use std::ffi::CStr;
use std::ptr::null_mut;

use crate::defs::otherffi::{
    begin_transaction, commit, duckdb_source_id, get_catalog, get_context, get_instance,
    new_connection, new_duckdb,
};
use crate::defs::{create_function_info, drop_create_function_info, QueryErrorContext};
use autocxx::prelude::*;
use cxx::let_cxx_string;

pub use crate::defs::otherffi::DatabaseInstance;
pub use crate::defs::{LogicalType, LogicalTypeId, ScalarFunction, ScalarFunctionBuilder};

mod defs;
mod macros;

pub fn get_version() -> String {
    unsafe {
        CStr::from_ptr(duckdb_source_id())
            .to_string_lossy()
            .to_string()
    }
}

pub fn load_extension(_instance: *mut DatabaseInstance) {
    unsafe {
        let db = new_duckdb();

        let mut instance = get_instance(&db);

        let catalog = get_catalog(&mut instance);

        let mut con = new_connection(&db);
        begin_transaction(&con);

        let context = get_context(&mut con);

        let_cxx_string!(function_name = "function_name");

        let mut logi = LogicalType::new(LogicalTypeId::VARCHAR).within_unique_ptr();

        moveit! {
            let mut builder = ScalarFunctionBuilder::new(
                &function_name,
                logi.pin_mut(),
            );
        }
        builder.as_mut().addArgument(logi.pin_mut());
        let _scalar_function = builder.as_mut().build();

        let info = create_function_info("function_name");

        let_cxx_string!(schema = "main");

        let ctx = QueryErrorContext::new(null_mut(), 0).within_box();

        let schema = &catalog.GetSchema(context, &schema, true, ctx);

        let context = get_context(&mut con);
        let catalog = get_catalog(&mut instance);

        catalog.CreateFunction1(context, *schema, info);

        commit(&con);

        drop_create_function_info(info);
    }
}
