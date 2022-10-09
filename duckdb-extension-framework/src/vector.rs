use crate::duckly::{
    duckdb_vector, duckdb_vector_assign_string_element_len, duckdb_vector_get_data,
};
use std::ffi::{c_char, c_void};

pub struct Vector(duckdb_vector);

impl Vector {
    pub fn get_data(&self) -> *mut c_void {
        unsafe { duckdb_vector_get_data(self.0) }
    }
}

impl From<duckdb_vector> for Vector {
    fn from(ptr: duckdb_vector) -> Self {
        Self(ptr)
    }
}

impl Vector {
    /// # Safety
    /// .
    pub unsafe fn assign_string_element_len(&self, p0: u64, p1: *const c_char, p2: u64) {
        duckdb_vector_assign_string_element_len(self.0, p0, p1, p2);
    }
}
