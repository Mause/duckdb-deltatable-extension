use crate::duckly::*;
use crate::{as_string, types, DuckDBType, FUNCTION_NAME};
use deltalake::open_table;
use libc::{free, strndup};
use std::ffi::{c_void, CStr, CString};
use std::mem::size_of;
use std::os::raw::c_char;
use std::slice;
use tokio::runtime::Runtime;

unsafe fn malloc_struct<T>() -> *mut T {
    duckdb_malloc(size_of::<T>() as u64).cast::<T>()
}

#[repr(C)]
struct MyBindDataStruct {
    size: i64,
    filename: *mut c_char,
}

#[repr(C)]
struct MyInitDataStruct {
    pos: i64,
}

/// # Safety
///
/// .
#[no_mangle]
unsafe extern "C" fn read_delta(info: duckdb_function_info, output: duckdb_data_chunk) {
    let bind_data = duckdb_function_get_bind_data(info) as *const MyBindDataStruct;
    let mut init_data = duckdb_function_get_init_data(info) as *mut MyInitDataStruct;
    let result_vector = duckdb_data_chunk_get_vector(output, 0);
    duckdb_vector_ensure_validity_writable(result_vector);
    let validity = duckdb_vector_get_validity(result_vector);

    let ptr = duckdb_vector_get_data(result_vector) as *mut i64;

    let size = (*bind_data).size;
    let result_slice = slice::from_raw_parts_mut(ptr, size as usize);

    let mut final_row: usize = 0;
    for (row, item) in result_slice
        .iter_mut()
        .enumerate()
        .take(duckdb_vector_size() as usize)
    {
        if (*init_data).pos >= size {
            break;
        }
        final_row = row + 1;
        *item = if (*init_data).pos % 2 == 0 { 42 } else { 84 };
        (*init_data).pos += 1;

        duckdb_validity_set_row_valid(validity, row as u64);
    }
    duckdb_data_chunk_set_size(output, final_row as u64);
}

unsafe extern "C" fn drop_my_bind_data_struct(v: *mut c_void) {
    let actual = v.cast::<MyBindDataStruct>();
    free((*actual).filename.cast());
    free(v);
}

/// # Safety
///
/// .
#[no_mangle]
unsafe extern "C" fn read_delta_bind(bind_info: duckdb_bind_info) {
    assert_eq!(duckdb_bind_get_parameter_count(bind_info), 1);

    let mut param = duckdb_bind_get_parameter(bind_info, 0);
    let ptr = duckdb_get_varchar(param);
    let cstring = CStr::from_ptr(ptr).to_str().unwrap();
    duckdb_destroy_value(&mut param);

    let handle = RUNTIME.block_on(open_table(cstring));
    if let Err(err) = handle {
        duckdb_bind_set_error(bind_info, as_string!(err.to_string()));
        return;
    }

    let table = handle.unwrap();
    let schema = table.schema().expect("no schema");
    for field in schema.get_fields() {
        let mut typ = duckdb_create_logical_type(types::map_type(field.get_type()) as u32);
        duckdb_bind_add_result_column(bind_info, as_string!(field.get_name()), typ);
        duckdb_destroy_logical_type(&mut typ);
    }

    let my_bind_data = malloc_struct::<MyBindDataStruct>();
    let string = CString::new(cstring).expect("c string");
    (*my_bind_data).filename = strndup(string.as_ptr().cast(), cstring.len());
    (*my_bind_data).size = 3;
    duckdb_bind_set_bind_data(
        bind_info,
        my_bind_data.cast(),
        Some(drop_my_bind_data_struct),
    );

    duckdb_free(ptr.cast::<c_void>());
}

/// # Safety
///
/// .
#[no_mangle]
unsafe extern "C" fn read_delta_init(info: duckdb_init_info) {
    // assert!(!duckdb_init_get_bind_data(info).is_null());
    // assert_eq!(duckdb_init_get_bind_data(null_mut()), null_mut());

    let mut my_init_data = malloc_struct::<MyInitDataStruct>();
    (*my_init_data).pos = 0;
    duckdb_init_set_init_data(info, my_init_data.cast(), Some(free));
}

pub unsafe fn build_table_function_def() -> *mut c_void {
    let table_function = duckdb_create_table_function();
    duckdb_table_function_set_name(table_function, FUNCTION_NAME.as_ptr().cast());
    let mut logical_type = duckdb_create_logical_type(DuckDBType::Varchar as u32);
    duckdb_table_function_add_parameter(table_function, logical_type);
    duckdb_destroy_logical_type(&mut logical_type);

    duckdb_table_function_set_function(table_function, Some(read_delta));
    duckdb_table_function_set_init(table_function, Some(read_delta_init));
    duckdb_table_function_set_bind(table_function, Some(read_delta_bind));
    table_function
}

lazy_static::lazy_static! {
    static ref RUNTIME: Runtime = tokio::runtime::Builder::new_current_thread()
            .build()
            .expect("runtime");
}
