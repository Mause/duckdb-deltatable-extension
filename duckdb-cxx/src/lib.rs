use std::ffi::CStr;
use std::ptr::{addr_of, null_mut};

use crate::defs::otherffi::{
    begin_transaction, commit, duckdb_source_id, get_catalog, get_context, get_instance,
    new_connection, new_duckdb, CreateFunctionInfo, DBConfig, Function,
};
use crate::defs::{real_create, QueryErrorContext};
use autocxx::prelude::*;
use cxx::let_cxx_string;

mod defs;
mod macros;

pub fn get_version() -> String {
    unsafe {
        CStr::from_ptr(duckdb_source_id())
            .to_string_lossy()
            .to_string()
    }
}

pub fn load_extension() {
    unsafe {
        // let_cxx_string!(x = ":memory:");

        println!("config names: {:?}", DBConfig::GetOptionNames());

        println!("cnt: {:?}", DBConfig::GetOptionCount());

        // let mut config = DBConfig::new().within_unique_ptr();
        // let cfg = get_raw_ptr!(config);

        let db = new_duckdb();

        let_cxx_string!(extension_name = "hello");

        // db.pin_mut().SetExtensionLoaded(&extension_name);
        // let is_loaded =
        //     crate::ffi::duckdb::DuckDB::ExtensionIsLoaded(db.deref_mut(), &extension_name);

        // println!("is loaded: {}", is_loaded);

        let mut instance = get_instance(&db);

        // let client_context = ClientContext::new(instance).within_unique_ptr();
        // let client_context = unpin(client_context);
        // let client_context = Box::into_pin(client_context);

        // let mut client_context = std::pin::Pin::into_inner_unchecked(client_context);
        // let mut client_context = MoveRef::into_inner(client_context);

        let mut catalog = get_catalog(&mut instance);

        let mut con = new_connection(&db);
        begin_transaction(&con);

        moveit! {
            let mut function = Function::new("prql_to_sql");
        }

        // set_name(from_pin_box(function), &extension_name);

        let context = get_context(&mut con);

        // let function = function.as_mut();

        println!("creating");
        let info = real_create();
        println!("created");

        // let_cxx_string!(fn_name = "function");

        // Box::pin(&mut (*info)).SetFunction(&fn_name);
        // <*const RustCreateFunctionInfo>::as_ref(info).unwrap().set_function(function);

        let_cxx_string!(schema = "main");

        let ctx = QueryErrorContext::new(null_mut(), 0).within_unique_ptr();

        let mut schema = &catalog.GetSchema(context, &schema, true, &Box::pin(*ctx));

        let mut catalog = get_catalog(&mut instance);
        let context = get_context(&mut con);
        catalog.CreateFunction1(context, *schema, addr_of!(info) as *mut CreateFunctionInfo);

        commit(&con);

        println!("Commited!");
    }
}

// unsafe impl New for RustCreateFunctionInfo {
//     type Output = RustCreateFunctionInfo;
//
//     unsafe fn new(self, this: Pin<&mut MaybeUninit<Self::Output>>) {
//         autocxx::moveit::new::by_raw(|x| {
//
//         });
//     }
// }
