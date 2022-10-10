use std::ffi::c_void;

use crate::duckly::{duckdb_init_info, duckdb_init_set_init_data};

pub struct InitInfo(duckdb_init_info);

impl From<duckdb_init_info> for InitInfo {
    fn from(ptr: duckdb_init_info) -> Self {
        Self(ptr)
    }
}

impl InitInfo {
    /// # Safety
    pub unsafe fn set_init_data(
        &self,
        data: *mut c_void,
        freeer: Option<unsafe extern "C" fn(*mut c_void)>,
    ) {
        duckdb_init_set_init_data(self.0, data, freeer);
    }
}
