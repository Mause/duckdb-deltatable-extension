extern crate core;

use std::ffi::{c_char, CStr, CString};
use std::pin::Pin;
use std::ptr::{null, null_mut};

pub use crate::defs::otherffi::{
    begin_transaction, commit, duckdb_source_id, get_catalog, get_context, new_connection, setBind,
    setFunction,
};
use autocxx::prelude::*;
use cxx::let_cxx_string;

pub use crate::defs::otherffi::DatabaseInstance;
pub use crate::defs::{
    DataChunk, ExpressionState, LogicalType, LogicalTypeId, QueryErrorContext,
    RustCreateFunctionInfo, ScalarFunction, ScalarFunctionBuilder, Value, Vector,
};

mod defs;
mod macros;

pub fn get_version() -> String {
    unsafe {
        CStr::from_ptr(duckdb_source_id())
            .to_string_lossy()
            .to_string()
    }
}

pub fn binder<'a>(
    args: &'a DataChunk,
    _state: &ExpressionState,
    result: Pin<&mut Vector>,
) -> *const c_char {
    use std::ops::Deref;
    let mut value = Value::from_string("hello");

    unsafe {
        let value = Pin::into_inner_unchecked(value.pin_mut());

        let result = Pin::get_unchecked_mut(result);

        result.reference_value(value);
    }

    let arg = args.get_value(0, 0).within_unique_ptr();

    let string = Value::get_string(arg);
    let string = string.deref().to_str().unwrap();

    if string == "frogs" {
        CString::new("hello").unwrap().into_raw()
    } else {
        null()
    }
}

/// # Safety
pub unsafe fn load_extension(ptr: *mut DatabaseInstance) {
    println!("ptr: {:?}", ptr);
    let instance = Pin::new_unchecked(ptr.as_mut().unwrap());
    println!("instance: {:?}", instance);
    let catalog = get_catalog(instance);

    let mut con = new_connection(Pin::new_unchecked(ptr.as_mut().unwrap()));
    begin_transaction(&con);

    let context = get_context(&mut con);

    let_cxx_string!(function_name = "function_name");

    let mut logi = LogicalType::new(LogicalTypeId::VARCHAR).within_unique_ptr();

    let mut builder =
        ScalarFunctionBuilder::new(&function_name, logi.pin_mut()).within_unique_ptr();
    setFunction(builder.pin_mut(), binder);
    builder.pin_mut().addArgument(logi.pin_mut());
    let scalar_function = builder.as_mut().unwrap().build();

    let info = RustCreateFunctionInfo::new(scalar_function);

    let_cxx_string!(schema = "main");

    let ctx = QueryErrorContext::new(null_mut(), 0).within_box();

    let schema = &catalog.GetSchema(context, &schema, true, ctx);

    let context = get_context(&mut con);
    let catalog = get_catalog(Pin::new_unchecked(ptr.as_mut().unwrap()));

    catalog.CreateFunction1(context, *schema, info.0);

    commit(&con);
}

#[cfg(test)]
mod test {
    use crate::defs::{get_instance, new_duckdb};
    use crate::load_extension;

    #[test]
    fn test_load() {
        unsafe {
            println!("hello!");

            let duckdb = new_duckdb();
            let instance = get_instance(&duckdb);
            load_extension(instance);
        };
    }
}