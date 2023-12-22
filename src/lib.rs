#![allow(dead_code)]
use duckdb::Connection;
use duckdb_loadable_macros::duckdb_entrypoint;
use libduckdb_sys as ffi;
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
#[duckdb_entrypoint]
pub fn deltatable_init_rust(conn: Connection) -> Result<(), Box<dyn Error>> {
    conn.register_table_function::<DeltaFunction>("read_delta")?;
    Ok(())
}
