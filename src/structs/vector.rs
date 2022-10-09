use duckdb_extension_framework::duckly::{
    duckdb_vector_assign_string_element_len, duckdb_vector_get_data,
};
use std::ffi::{c_char, c_void};

pub struct Vector(*mut c_void);

impl Vector {
    pub(crate) fn get_data(&self) -> *mut c_void {
        unsafe { duckdb_vector_get_data(self.0) }
    }
}

impl From<*mut c_void> for Vector {
    fn from(ptr: *mut c_void) -> Self {
        Self(ptr)
    }
}

impl Vector {
    pub(crate) fn assign_string_element_len(&self, p0: u64, p1: *const c_char, p2: u64) {
        unsafe {
            duckdb_vector_assign_string_element_len(self.0, p0, p1, p2);
        }
    }
}
