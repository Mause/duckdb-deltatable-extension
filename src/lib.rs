#![allow(dead_code)]
use duckdb_extension_framework::Database;
use std::{
    error::Error,
    ffi::{c_char, c_void},
};

use crate::table_function::build_table_function_def;
use duckdb_extension_framework::duckly::duckdb_library_version;

mod table_function;
mod types;

/// Init hook for DuckDB, registers all functionality provided by this extension
/// # Safety
/// .
#[no_mangle]
pub unsafe extern "C" fn deltatable_init_rust(db: *mut c_void) {
    init(db).expect("init failed");
}

unsafe fn init(db: *mut c_void) -> Result<(), Box<dyn Error>> {
    let db = Database::from_cpp_duckdb(db);
    let table_function = build_table_function_def();
    let connection = db.connect()?;
    connection.register_table_function(table_function)?;
    Ok(())
}

/// Version hook for DuckDB, indicates which version of DuckDB this extension was compiled against
#[no_mangle]
pub extern "C" fn deltatable_version_rust() -> *const c_char {
    unsafe { duckdb_library_version() }
}
