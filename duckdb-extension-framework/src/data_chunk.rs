use crate::duckly::{duckdb_data_chunk_get_vector, duckdb_data_chunk_set_size};
use crate::Vector;
use std::ffi::c_void;

pub struct DataChunk {
    ptr: *mut c_void,
}

impl DataChunk {
    pub fn get_vector(&self, p0: u64) -> Vector {
        Vector::from(unsafe { duckdb_data_chunk_get_vector(self.ptr, p0) })
    }
    pub fn set_size(&self, size: u64) {
        unsafe { duckdb_data_chunk_set_size(self.ptr, size) };
    }
}

impl From<*mut c_void> for DataChunk {
    fn from(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}
