use crate::duckly::{
    duckdb_function_get_bind_data, duckdb_function_get_init_data, duckdb_function_set_error,
};
use std::ffi::c_void;
use std::os::raw::c_char;

pub struct FunctionInfo(*mut c_void);

impl FunctionInfo {
    pub(crate) fn set_error(&self, p0: *const c_char) {
        unsafe {
            duckdb_function_set_error(self.0, p0);
        }
    }
    pub(crate) fn get_bind_data<T>(&self) -> *mut T {
        unsafe { duckdb_function_get_bind_data(self.0) as *mut T }
    }
    pub(crate) fn get_init_data<T>(&self) -> *mut T {
        unsafe { duckdb_function_get_init_data(self.0) as *mut T }
    }
}

impl From<*mut c_void> for FunctionInfo {
    fn from(ptr: *mut c_void) -> Self {
        Self(ptr)
    }
}
