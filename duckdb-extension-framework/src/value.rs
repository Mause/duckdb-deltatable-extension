use crate::duckly::{duckdb_destroy_value, duckdb_get_varchar, duckdb_value};
use std::ffi::CString;
use std::ptr::addr_of_mut;

pub struct Value {
    ptr: *mut duckdb_value,
}

impl Value {
    pub fn get_varchar(&self) -> CString {
        unsafe { CString::from_raw(duckdb_get_varchar(self.ptr as u64)) }
    }
}

impl From<u64> for Value {
    fn from(ptr: u64) -> Self {
        Self {
            ptr: ptr as *mut duckdb_value,
        }
    }
}

impl Drop for Value {
    fn drop(&mut self) {
        unsafe {
            duckdb_destroy_value(addr_of_mut!(self.ptr).cast());
        }
    }
}
