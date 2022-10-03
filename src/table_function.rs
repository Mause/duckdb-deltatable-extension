use crate::duckly::*;
use crate::{as_string, types, DuckDBType, FUNCTION_NAME};
use deltalake::open_table;
use libc::{free, strndup};
use std::ffi::{c_void, CStr, CString};
use std::fmt::Display;
use std::fs::File;
use std::mem::size_of;
use std::os::raw::c_char;
use std::path::Path;
use std::ptr::null_mut;
use std::slice;
use tokio::runtime::Runtime;

use parquet::file::reader::SerializedFileReader;
use parquet::record::Field;

unsafe fn malloc_struct<T>() -> *mut T {
    duckdb_malloc(size_of::<T>() as u64).cast::<T>()
}

#[repr(C)]
struct MyBindDataStruct {
    filename: *mut c_char,
}

#[repr(C)]
struct MyInitDataStruct {
    done: bool, // TODO: support more than *vector size* rows
}

/// # Safety
///
/// .
#[no_mangle]
unsafe extern "C" fn read_delta(info: duckdb_function_info, output: duckdb_data_chunk) {
    let bind_data = duckdb_function_get_bind_data(info) as *const MyBindDataStruct;
    let mut init_data = duckdb_function_get_init_data(info).cast::<MyInitDataStruct>();

    let filename = CStr::from_ptr((*bind_data).filename);

    let table_result = RUNTIME.block_on(open_table(filename.to_str().unwrap()));

    if let Err(err) = table_result {
        duckdb_function_set_error(info, as_string!(err.to_string()));
        return;
    }

    let table = table_result.unwrap();

    let root_dir = Path::new(filename.to_str().unwrap());
    let mut final_row: usize = 0;
    for pq_filename in table.get_files_iter() {
        if (*init_data).done {
            break;
        }
        let reader =
            SerializedFileReader::new(File::open(root_dir.join(pq_filename)).unwrap()).unwrap();

        for row in reader {
            for (idx, (_key, value)) in row.get_column_iter().enumerate() {
                match value {
                    Field::Int(v) => {
                        assign(output, final_row, idx, *v);
                    }
                    Field::Bool(v) => {
                        assign(output, final_row, idx, *v);
                    }
                    Field::Long(v) => {
                        assign(output, final_row, idx, *v);
                    }
                    // TODO: support more types
                    _ => panic!("{:?} is unsupported", value),
                }
            }
            final_row += 1;
        }
    }
    (*init_data).done = true;
    duckdb_data_chunk_set_size(output, final_row as u64);
}

unsafe fn assign<T: Display + Copy + 'static>(
    output: *mut c_void,
    final_row: usize,
    idx: usize,
    v: T,
) {
    get_column_result_vector::<T>(output, idx)[final_row] = v;
}

unsafe fn get_column_result_vector<T>(
    output: *mut c_void,
    column_index: usize,
) -> &'static mut [T] {
    let result_vector = duckdb_data_chunk_get_vector(output, column_index as u64);
    let ptr = duckdb_vector_get_data(result_vector).cast::<T>();
    slice::from_raw_parts_mut(ptr, duckdb_vector_size() as usize)
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
    assert!(!duckdb_init_get_bind_data(info).is_null());
    assert_eq!(duckdb_init_get_bind_data(null_mut()), null_mut());

    let mut my_init_data = malloc_struct::<MyInitDataStruct>();
    (*my_init_data).done = false;
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
