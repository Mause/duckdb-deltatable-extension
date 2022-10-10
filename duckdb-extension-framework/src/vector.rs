use crate::{
    duckly::{
        duckdb_vector, duckdb_vector_assign_string_element,
        duckdb_vector_assign_string_element_len, duckdb_vector_ensure_validity_writable,
        duckdb_vector_get_column_type, duckdb_vector_get_data, duckdb_vector_get_validity, idx_t,
    },
    LogicalType,
};
use std::ffi::{c_char, c_void};

pub struct Vector(duckdb_vector);

impl From<duckdb_vector> for Vector {
    fn from(ptr: duckdb_vector) -> Self {
        Self(ptr)
    }
}

impl Vector {
    pub fn get_data(&self) -> *mut c_void {
        unsafe { duckdb_vector_get_data(self.0) }
    }

    /// Assigns a string element in the vector at the specified location.
    ///
    /// # Arguments
    ///  * `index` - The row position in the vector to assign the string to
    ///  * `str` - The string
    ///  * `str_len` - The length of the string (in bytes)
    ///
    /// # Safety
    pub unsafe fn assign_string_element_len(
        &self,
        index: idx_t,
        str_: *const c_char,
        str_len: idx_t,
    ) {
        duckdb_vector_assign_string_element_len(self.0, index, str_, str_len);
    }

    /// Assigns a string element in the vector at the specified location.
    ///
    /// # Arguments
    ///  * `index` - The row position in the vector to assign the string to
    ///  * `str` - The null-terminated string"]
    ///
    /// # Safety
    pub unsafe fn assign_string_element(&self, index: idx_t, str_: *const c_char) {
        duckdb_vector_assign_string_element(self.0, index, str_);
    }
    pub fn get_column_type(&self) -> LogicalType {
        unsafe { LogicalType::from(duckdb_vector_get_column_type(self.0)) }
    }
    pub fn get_validity(&self) -> *mut u64 {
        unsafe { duckdb_vector_get_validity(self.0) }
    }
    pub fn ensure_validity_writable(&self) {
        unsafe { duckdb_vector_ensure_validity_writable(self.0) };
    }
}
