use crate::constants::DuckDBType;
use crate::duckly::{duckdb_create_logical_type, duckdb_destroy_logical_type};
use std::ffi::c_void;

#[derive(Debug)]
pub struct LogicalType {
    pub(crate) typ: *mut c_void,
}

impl LogicalType {
    pub fn new(typ: DuckDBType) -> Self {
        unsafe {
            Self {
                typ: duckdb_create_logical_type(typ as u32),
            }
        }
    }
}

impl Drop for LogicalType {
    fn drop(&mut self) {
        unsafe {
            duckdb_destroy_logical_type(&mut self.typ);
        }
    }
}
