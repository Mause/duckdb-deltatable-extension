use crate::duckly::{
    duckdb_vector, duckdb_vector_assign_string_element_len, duckdb_vector_get_data, idx_t,
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
    /// Assigns a string element in the vector at the specified location.
    ///
    /// # Arguments
    ///  * `index` - The row position in the vector to assign the string to
    ///  * `str` - The string
    ///  * `str_len` - The length of the string (in bytes)
    ///
    /// # Safety
    /// .
    pub unsafe fn assign_string_element_len(
        &self,
        index: idx_t,
        str_: *const c_char,
        str_len: idx_t,
    ) {
        duckdb_vector_assign_string_element_len(self.0, index, str_, str_len);
    }
}
