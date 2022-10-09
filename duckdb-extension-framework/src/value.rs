use crate::duckly::{duckdb_destroy_value, duckdb_get_varchar, duckdb_value};
use std::ffi::CString;

pub struct Value(duckdb_value);

impl Value {
    pub fn get_varchar(&self) -> CString {
        unsafe { CString::from_raw(duckdb_get_varchar(self.0)) }
    }
}

impl From<duckdb_value> for Value {
    fn from(ptr: duckdb_value) -> Self {
        Self(ptr)
    }
}

impl Drop for Value {
    fn drop(&mut self) {
        unsafe {
            duckdb_destroy_value(&mut self.0);
        }
    }
}
