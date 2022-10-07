#![allow(dead_code)]
use crate::constants::{DuckDBType, FUNCTION_NAME};
use std::os::raw::c_char;
use std::ptr::{addr_of, null_mut};

use crate::duckly::*;
use crate::table_function::build_table_function_def;

mod constants;
mod duckly;
mod error;
mod structs;
mod table_function;
mod types;

/// Equivalent of [`DatabaseData`](https://github.com/duckdb/duckdb/blob/50951241de3d9c06fac5719dcb907eb21163dcab/src/include/duckdb/main/capi_internal.hpp#L27), wraps `duckdb::DuckDB`
#[repr(C)]
struct Wrapper {
    instance: *const u8,
}

/// Init hook for DuckDB, registers all functionality provided by this extension
#[no_mangle]
pub extern "C" fn deltatable_init_rust(db: *mut u8) {
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

/// Version hook for DuckDB, indicates which version of DuckDB this extension was compiled against
#[no_mangle]
pub extern "C" fn deltatable_version_rust() -> *const c_char {
    unsafe { duckdb_library_version() }
}
