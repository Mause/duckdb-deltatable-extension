use crate::defs::ffi::duckdb::ConfigurationOption;
use autocxx::prelude::*;
use cxx::private::VectorElement;
use cxx::CxxVector;
use cxx::{type_id, ExternType, SharedPtr};
use std::fmt::Formatter;
use std::mem::MaybeUninit;

pub(crate) struct TaskScheduler {}
unsafe impl ExternType for TaskScheduler {
    type Id = type_id!("duckdb::TaskScheduler");
    type Kind = cxx::kind::Opaque;
}

include_cpp! {
    #include "duckdb.hpp"
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
}

include_cpp! {
    #include "RustCreateFunctionInfo.h"
    name!(wrapper)
    generate!("ext_framework::RustCreateFunctionInfo")
    generate!("ext_framework::create")
}

pub(crate) type RustCreateFunctionInfo =
    crate::defs::wrapper::ext_framework::RustCreateFunctionInfo;

pub(crate) type QueryErrorContext = crate::defs::ffi::duckdb::QueryErrorContext;

pub(crate) unsafe fn real_create() -> SharedPtr<RustCreateFunctionInfo> {
    crate::defs::wrapper::ext_framework::create()
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

        pub(crate) type Function = crate::defs::ffi::duckdb::Function;
        pub(crate) type DBConfig = crate::defs::ffi::duckdb::DBConfig;
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
