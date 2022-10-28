#![allow(clippy::needless_lifetimes)]
#![allow(clippy::upper_case_acronyms)]

use crate::defs::ffi::{duckdb::ConfigurationOption, ToCppString};
use crate::DatabaseInstance;
use autocxx::prelude::*;
use cxx::private::VectorElement;
use cxx::{type_id, ExternType};
use cxx::{CxxVector, SharedPtr};
use std::any::Any;
use std::fmt::{Debug, Formatter};
use std::mem::MaybeUninit;
use std::pin::Pin;

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
    generate!("duckdb::RustCreateFunctionInfo")
    generate!("duckdb::create_function_info")
    generate!("duckdb::drop_create_function_info")
    generate!("duckdb::create_logical_type")
    generate!("duckdb::LogicalTypeId")
    generate!("duckdb::ScalarFunctionBuilder")
    generate!("duckdb::new_duckdb")
    generate!("duckdb::get_instance")
    generate!("duckdb::ExpressionState")
    generate!("duckdb::vector_print")
    generate!("duckdb::vector_reference_value")
}

use self::ffi::duckdb;

pub(crate) type QueryErrorContext = duckdb::QueryErrorContext;
pub type ScalarFunction = duckdb::ScalarFunction;
pub type ScalarFunctionBuilder = duckdb::ScalarFunctionBuilder;
pub type LogicalTypeId = duckdb::LogicalTypeId;
pub type LogicalType = duckdb::LogicalType;
pub type DuckDB = duckdb::DuckDB;
pub type DataChunk = otherffi::DataChunk;
pub type ExpressionState = duckdb::ExpressionState;
pub type Vector = otherffi::Vector;

pub fn new_duckdb() -> SharedPtr<DuckDB> {
    unsafe { duckdb::new_duckdb() }
}
pub fn get_instance(duckdb: &SharedPtr<DuckDB>) -> *mut DatabaseInstance {
    unsafe { duckdb::get_instance(duckdb) }
}

pub(crate) struct RustCreateFunctionInfo(pub(crate) *mut CreateFunctionInfo);
impl RustCreateFunctionInfo {
    pub fn new(function_name: impl ToCppString) -> Self {
        Self(unsafe { duckdb::create_function_info(function_name) })
    }
}
impl Drop for RustCreateFunctionInfo {
    fn drop(&mut self) {
        unsafe {
            duckdb::drop_create_function_info(self.0);
        }
    }
}

impl Vector {
    pub fn print(&self) {
        unsafe {
            duckdb::vector_print(self);
        }
    }
}

pub type ScalarFunctionT = for<'r, 's, 't0> fn(
    &'r duckdb::DataChunk,
    &'s duckdb::ExpressionState,
    Pin<&'t0 mut otherffi::Vector>,
);

impl Debug for DatabaseInstance {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DatabaseInstance")
            .field("type_id", &self.type_id())
            .finish()
    }
}

impl LogicalType {
    pub unsafe fn new(id: LogicalTypeId) -> impl autocxx::moveit::new::New<Output = Self> {
        autocxx::moveit::new::by_raw(move |this| {
            let this = this.get_unchecked_mut().as_mut_ptr();
            otherffi::duckdb_LogicalType_new1_autocxx_wrapper(this, id)
        })
    }
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

        pub type DatabaseInstance = crate::defs::ffi::duckdb::DatabaseInstance;
        pub(crate) type CreateFunctionInfo = crate::defs::ffi::duckdb::CreateFunctionInfo;
        pub(crate) type Catalog = crate::defs::ffi::duckdb::Catalog;
        pub(crate) type ClientContext = crate::defs::ffi::duckdb::ClientContext;
        pub(crate) type LogicalType = crate::defs::ffi::duckdb::LogicalType;
        pub(crate) type LogicalTypeId = crate::defs::ffi::duckdb::LogicalTypeId;
        type ScalarFunctionBuilder = crate::defs::ScalarFunctionBuilder;
        pub(crate) type DataChunk = crate::defs::ffi::duckdb::DataChunk;
        type ExpressionState = crate::defs::ExpressionState;
        pub(crate) type Vector = crate::defs::ffi::duckdb::Vector;

        pub(crate) type Connection;

        pub fn duckdb_source_id() -> *const c_char;

        pub fn new_connection(duckdb: Pin<&mut DatabaseInstance>) -> SharedPtr<Connection>;
        pub fn begin_transaction(conn: &SharedPtr<Connection>);
        pub fn commit(conn: &SharedPtr<Connection>);
        pub fn get_catalog(conn: Pin<&mut DatabaseInstance>) -> Pin<&mut Catalog>;
        pub fn get_context(conn: &mut SharedPtr<Connection>) -> Pin<&mut ClientContext>;

        pub(crate) unsafe fn duckdb_LogicalType_new1_autocxx_wrapper(
            autocxx_gen_this: *mut LogicalType,
            arg1: LogicalTypeId,
        );

        pub(crate) unsafe fn setBind(
            autocxx_gen_this: Pin<&mut ScalarFunctionBuilder>,
            scalar_function: unsafe extern "C" fn(
                args: &DataChunk,
                state: &ExpressionState,
                result: Pin<&mut Vector>,
            ),
        );
        pub(crate) unsafe fn setFunction(
            autocxx_gen_this: Pin<&mut ScalarFunctionBuilder>,
            scalar_function: unsafe extern "C" fn(
                args: &DataChunk,
                state: &ExpressionState,
                result: Pin<&mut Vector>,
            ),
        );
    }
}
