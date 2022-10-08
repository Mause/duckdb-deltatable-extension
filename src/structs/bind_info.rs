use crate::duckly::{
    duckdb_bind_add_result_column, duckdb_bind_get_parameter, duckdb_bind_get_parameter_count,
    duckdb_bind_info, duckdb_bind_set_bind_data, duckdb_bind_set_error,
};
use crate::structs::logical_type::LogicalType;
use crate::structs::value::Value;
use std::ffi::c_void;
use std::os::raw::c_char;

pub struct BindInfo {
    ptr: *mut c_void,
}

impl BindInfo {
    pub fn add_result_column(&self, p0: *const c_char, p1: LogicalType) {
        unsafe {
            duckdb_bind_add_result_column(self.ptr, p0, p1.typ);
        }
    }
}

impl BindInfo {
    pub fn set_error(&self, p0: *const c_char) {
        unsafe {
            duckdb_bind_set_error(self.ptr, p0.cast());
        }
    }
}

impl BindInfo {
    pub fn set_bind_data(
        &self,
        data: *mut c_void,
        free_function: Option<unsafe extern "C" fn(*mut c_void)>,
    ) {
        unsafe {
            duckdb_bind_set_bind_data(self.ptr, data, free_function);
        }
    }
}

impl BindInfo {
    pub fn get_parameter_count(&self) -> u64 {
        unsafe { duckdb_bind_get_parameter_count(self.ptr) }
    }
    pub fn get_parameter(&self, param_index: u64) -> Value {
        unsafe { Value::from(duckdb_bind_get_parameter(self.ptr, param_index)) }
    }
}

impl From<duckdb_bind_info> for BindInfo {
    fn from(ptr: duckdb_bind_info) -> Self {
        Self { ptr }
    }
}