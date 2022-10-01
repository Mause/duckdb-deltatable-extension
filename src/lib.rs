use std::ffi::c_void;
use std::mem::size_of;
use std::ptr::{null, null_mut};

use crate::duckly::duckdb_connect;
use crate::duckly::duckdb_connection;
use crate::duckly::duckdb_database;
use crate::duckly::duckdb_get_varchar;
use crate::duckly::duckdb_open;
use crate::duckly::duckdb_result_get_chunk;
use crate::duckly::duckdb_value;
use crate::duckly::{
    duckdb_add_replacement_scan, duckdb_query, duckdb_replacement_scan_info,
    duckdb_replacement_scan_set_function_name, size_t, u_int8_t,
};

mod duckly;

#[repr(C)]
pub struct Wrapper {
    instance: *const u8,
}

pub extern "C" fn replacement(
    info: duckdb_replacement_scan_info,
    table_name: *const u_int8_t,
    data: *mut c_void,
) {
    unsafe {
        duckdb_replacement_scan_set_function_name(info, "read_delta".as_ptr());
        // let val = duckdb_create_int64(42);
        // duckdb_replacement_scan_add_parameter(info, val);
        // duckdb_destroy_value(val.);
    }
}

#[no_mangle]
pub extern "C" fn libtest_extension_init(db: *mut u8) {
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

unsafe fn alloc<T: Sized>() -> *mut T {
    libc::malloc(size_of::<i8>()) as *mut T
}

#[no_mangle]
pub extern "C" fn libtest_extension_version() -> *mut u_int8_t {
    unsafe {
        let mut database: duckdb_database = null_mut();
        let mut connection: duckdb_connection = null_mut();
        let result = alloc();
        let value = alloc::<duckdb_value>();

        duckdb_open(null(), &mut database);
        duckdb_connect(database, &mut connection);
        duckdb_query(connection, "pragma version".as_ptr(), result);

        duckdb_result_get_chunk(*result, 0);

        return duckdb_get_varchar(value as u64);
    }
}
