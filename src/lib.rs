#![allow(dead_code)]
use crate::constants::{DuckDBType, FUNCTION_NAME};
use std::os::raw::c_char;
use std::ptr::{addr_of, null_mut};

use crate::duckly::*;
use crate::table_function::build_table_function_def;

mod constants;
mod duckly;
mod error;
mod table_function;
mod types;

#[repr(C)]
struct Wrapper {
    instance: *const u8,
}

#[no_mangle]
pub extern "C" fn libtest_extension_init_rust(db: *mut u8) {
    unsafe {
        let wrap = Wrapper { instance: db };

        let real_db = addr_of!(wrap) as duckdb_database;

        let mut table_function = build_table_function_def();

        let mut connection: duckdb_connection = null_mut();
        check!(duckdb_connect(real_db, &mut connection));
        check!(duckdb_register_table_function(connection, table_function));
        duckdb_disconnect(&mut connection);

        duckdb_destroy_table_function(&mut table_function);
    }
}

#[no_mangle]
pub extern "C" fn libtest_extension_version_rust() -> *const c_char {
    unsafe { DuckDB_LibraryVersion() }
}
