#![allow(dead_code)]
use duckdb::ffi::{duckdb_database, duckdb_library_version};
use duckdb::Connection;
use std::{
    error::Error,
    ffi::{c_char, c_void},
};

use crate::table_function::DeltaFunction;

mod table_function;
mod types;

/// Init hook for DuckDB, registers all functionality provided by this extension
/// # Safety
/// .
#[no_mangle]
pub unsafe extern "C" fn deltatable_init_rust(db: *mut c_void) {
    init(db.cast()).expect("init failed");
}

unsafe fn init(db: duckdb_database) -> Result<(), Box<dyn Error>> {
    let connection = Connection::from_cpp(db)?;
    connection.register_table_function::<DeltaFunction>("read_delta")?;
    Ok(())
}

/// Version hook for DuckDB, indicates which version of DuckDB this extension was compiled against
#[no_mangle]
pub extern "C" fn deltatable_version_rust() -> *const c_char {
    unsafe { duckdb_library_version() }
}
