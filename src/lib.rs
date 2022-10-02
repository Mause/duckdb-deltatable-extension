use std::ffi::{c_void, CString};
use std::os::raw::c_char;
use std::ptr::{addr_of_mut, null, null_mut};

use crate::duckly::{
    duckdb_add_replacement_scan, duckdb_close, duckdb_connect, duckdb_connection,
    duckdb_data_chunk_get_vector, duckdb_database, duckdb_destroy_data_chunk,
    duckdb_destroy_logical_type, duckdb_destroy_result, duckdb_disconnect, duckdb_get_type_id,
    duckdb_open, duckdb_query, duckdb_replacement_scan_info,
    duckdb_replacement_scan_set_function_name, duckdb_result, duckdb_result_get_chunk,
    duckdb_vector_get_column_type, duckdb_vector_get_data,
};

mod duckly;
mod error;
mod strings;

#[repr(C)]
pub struct Wrapper {
    instance: *const u8,
}

/// # Safety
/// This function should only be called directly by DuckDB
pub unsafe extern "C" fn replacement(
    info: duckdb_replacement_scan_info,
    _table_name: *const c_char,
    _data: *mut c_void,
) {
    duckdb_replacement_scan_set_function_name(info, "read_delta".as_ptr() as *const c_char);
    // let val = duckdb_create_int64(42);
    // duckdb_replacement_scan_add_parameter(info, val);
    // duckdb_destroy_value(val.);
}

#[no_mangle]
pub extern "C" fn libtest_extension_init_rust(db: *mut u8) {
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

#[no_mangle]
pub extern "C" fn libtest_extension_version_rust() -> *const c_char {
    unsafe {
        let mut database: duckdb_database = null_mut();
        let mut connection: duckdb_connection = null_mut();
        let mut result = duckdb_result::default();

        check!(duckdb_open(null(), &mut database));
        check!(duckdb_connect(database, &mut connection));
        let string = CString::new("pragma version").expect("bad cString");
        check!(duckdb_query(
            connection,
            string.as_ptr() as *const c_char,
            addr_of_mut!(result),
        ));
        let mut chunk = duckdb_result_get_chunk(result, 0);
        let vect = duckdb_data_chunk_get_vector(chunk, 0);

        let mut column_type = duckdb_vector_get_column_type(vect);
        assert_eq!(duckdb_get_type_id(column_type), 17);
        duckdb_destroy_logical_type(addr_of_mut!(column_type));

        let data = duckdb_vector_get_data(vect);

        let res = strings::convert_string(data, 1);

        duckdb_destroy_data_chunk(&mut chunk);
        duckdb_destroy_result(&mut result);

        duckdb_disconnect(&mut connection);
        duckdb_close(&mut database);

        strings::static_version_string(res)
    }
}
