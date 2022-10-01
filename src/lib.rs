use std::ffi::{c_void, CString};
use std::mem::size_of;
use std::os::raw::c_char;
use std::ptr::{addr_of_mut, null, null_mut};
use std::slice;

use crate::duckly::{
    duckdb_add_replacement_scan, duckdb_connect, duckdb_connection, duckdb_data_chunk_get_vector,
    duckdb_database, duckdb_get_type_id, duckdb_open, duckdb_query, duckdb_replacement_scan_info,
    duckdb_replacement_scan_set_function_name, duckdb_result, duckdb_result_get_chunk,
    duckdb_state, duckdb_state_DuckDBError, duckdb_vector_get_column_type, duckdb_vector_get_data,
};

mod duckly;

#[repr(C)]
pub struct Wrapper {
    instance: *const u8,
}

#[repr(C)]
struct duckdb_string_t {
    length: u32,
    data: *const c_char,
}

pub extern "C" fn replacement(
    info: duckdb_replacement_scan_info,
    table_name: *const c_char,
    data: *mut c_void,
) {
    unsafe {
        duckdb_replacement_scan_set_function_name(info, "read_delta".as_ptr() as *const c_char);
        // let val = duckdb_create_int64(42);
        // duckdb_replacement_scan_add_parameter(info, val);
        // duckdb_destroy_value(val.);
    }
}

#[no_mangle]
pub extern "C" fn libtest_extension_init_v2(db: *mut u8) {
    unsafe {
        let real_db = Wrapper { instance: db };

        duckdb_add_replacement_scan(
            real_db.instance as duckdb_database,
            Some(replacement),
            null_mut(),
            None,
        );
    }
}

const STRING_INLINE_LENGTH: i32 = 12;

unsafe fn convert_string<'f>(val: *const c_void, idx: usize) -> CString {
    assert!(idx >= 1);

    println!("vak: {:?}", val);
    let base_ptr = val.offset(((idx - 1) * size_of::<duckdb_string_t>()) as isize);
    println!("base ptr: {:?}", base_ptr);
    let length_ptr = base_ptr as *const i32;
    let length = *length_ptr;
    println!("length: {}", length);
    if length <= STRING_INLINE_LENGTH {
        let prefix_ptr = base_ptr.offset(size_of::<i32>() as isize);
        return unsafe_string(prefix_ptr as *const u8, length);
    } else {
        println!("not an inline string");
        let ptr_ptr = base_ptr.offset((size_of::<i32>() * 2) as isize) as *const *const u8;
        println!("ptr ptr: {:?}", ptr_ptr);
        let data_ptr = *ptr_ptr;
        println!("data ptr: {:?}", data_ptr);
        return unsafe_string(data_ptr, length);
    }
}

unsafe fn unsafe_string<'f>(ptr: *const u8, len: i32) -> CString {
    println!("about to slice");
    let slice = slice::from_raw_parts(ptr, len as usize);
    println!("slice: {:?}", slice);

    let cow = CString::from_vec_unchecked(slice.clone().to_vec());

    println!("cowd: {:?}", cow);

    return cow;
}

#[no_mangle]
pub extern "C" fn libtest_extension_version_v2() -> CString {
    unsafe {
        let mut database: duckdb_database = null_mut();
        let mut connection: duckdb_connection = null_mut();
        let mut result = duckdb_result::default();

        check(duckdb_open(null(), &mut database));
        check(duckdb_connect(database, &mut connection));
        let string = CString::new("pragma version").expect("bad cString");
        check(duckdb_query(
            connection,
            string.as_ptr() as *const c_char,
            addr_of_mut!(result),
        ));
        let chunk = duckdb_result_get_chunk(result, 0);
        let vect = duckdb_data_chunk_get_vector(chunk, 0);

        let column_type = duckdb_vector_get_column_type(vect);
        assert_eq!(duckdb_get_type_id(column_type), 17);

        let data = duckdb_vector_get_data(vect);

        return convert_string(data, 1);
    }
}

fn check(p0: duckdb_state) {
    if p0 == duckdb_state_DuckDBError {
        panic!("Duckdb error");
    }
}
