#![allow(clippy::needless_lifetimes)]
#![allow(clippy::upper_case_acronyms)]

use crate::defs::ffi::{duckdb::ConfigurationOption, ToCppString};
use autocxx::prelude::*;
use cxx::private::VectorElement;
use cxx::CxxVector;
use cxx::{type_id, ExternType};
use std::fmt::Formatter;
use std::mem::MaybeUninit;

use self::otherffi::CreateFunctionInfo;

pub(crate) struct TaskScheduler {}
unsafe impl ExternType for TaskScheduler {
    type Id = type_id!("duckdb::TaskScheduler");
    type Kind = cxx::kind::Opaque;
}

include_cpp! {
    #include "wrapper.hpp"
    generate!("duckdb::DuckDB")
    generate!("duckdb::DBConfig")
    generate!("duckdb::ConfigurationOption")
    generate!("duckdb::Function")
    generate!("duckdb::Catalog")
    generate!("duckdb::ClientContext")
    generate!("duckdb::CreateFunctionInfo")
    generate!("duckdb::CreateInfo")
    generate!("duckdb::CatalogType")
    generate!("duckdb::QueryErrorContext")
    extern_cpp_type!("duckdb::TaskScheduler", crate::TaskScheduler)
    generate!("ext_framework::RustCreateFunctionInfo")
    generate!("ext_framework::create_function_info")
    generate!("ext_framework::drop_create_function_info")
}

pub(crate) type QueryErrorContext = crate::defs::ffi::duckdb::QueryErrorContext;

pub(crate) unsafe fn create_function_info(
    function_name: impl ToCppString,
) -> *mut CreateFunctionInfo {
    crate::defs::ffi::ext_framework::create_function_info(function_name)
}
pub(crate) unsafe fn drop_create_function_info(ptr: *mut CreateFunctionInfo) {
    crate::defs::ffi::ext_framework::drop_create_function_info(ptr);
}

unsafe impl VectorElement for ConfigurationOption {
    fn __typename(_f: &mut Formatter) -> std::fmt::Result {
        todo!()
    }

    fn __vector_size(_v: &CxxVector<Self>) -> usize {
        todo!()
    }

    unsafe fn __get_unchecked(_v: *mut CxxVector<Self>, _poss: usize) -> *mut Self {
        todo!()
    }

    fn __unique_ptr_null() -> MaybeUninit<*mut std::ffi::c_void> {
        todo!()
    }

    unsafe fn __unique_ptr_raw(_raw: *mut CxxVector<Self>) -> MaybeUninit<*mut std::ffi::c_void> {
        todo!()
    }

    unsafe fn __unique_ptr_get(
        _repr: MaybeUninit<*mut std::ffi::c_void>,
    ) -> *const CxxVector<Self> {
        todo!()
    }

    unsafe fn __unique_ptr_release(
        _repr: MaybeUninit<*mut std::ffi::c_void>,
    ) -> *mut CxxVector<Self> {
        todo!()
    }

    unsafe fn __unique_ptr_drop(_repr: MaybeUninit<*mut std::ffi::c_void>) {
        todo!()
    }
}

#[cxx::bridge(namespace = "duckdb")]
pub mod otherffi {
    unsafe extern "C++" {
        include!("wrapper.hpp");
        include!("duckdb.hpp");

        pub(crate) type DatabaseInstance = crate::defs::ffi::duckdb::DatabaseInstance;
        pub(crate) type CreateFunctionInfo = crate::defs::ffi::duckdb::CreateFunctionInfo;
        pub(crate) type DuckDB = crate::defs::ffi::duckdb::DuckDB;
        pub(crate) type Catalog = crate::defs::ffi::duckdb::Catalog;
        pub(crate) type ClientContext = crate::defs::ffi::duckdb::ClientContext;

        pub(crate) type Connection;

        pub(crate) fn new_duckdb() -> SharedPtr<DuckDB>;
        pub(crate) fn duckdb_source_id() -> *const c_char;
        pub(crate) fn get_instance(buf: &SharedPtr<DuckDB>) -> SharedPtr<DatabaseInstance>;

        pub(crate) fn new_connection(duckdb: &SharedPtr<DuckDB>) -> SharedPtr<Connection>;
        pub(crate) fn begin_transaction(conn: &SharedPtr<Connection>);
        pub(crate) fn commit(conn: &SharedPtr<Connection>);
        pub(crate) fn get_catalog(conn: &mut SharedPtr<DatabaseInstance>) -> Pin<&mut Catalog>;
        pub(crate) fn get_context(conn: &mut SharedPtr<Connection>) -> Pin<&mut ClientContext>;
    }
}
