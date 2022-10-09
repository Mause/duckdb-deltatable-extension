use crate::check;
use crate::structs::table_function::TableFunction;
use duckdb_extension_framework::duckly::{
    duckdb_connect, duckdb_connection, duckdb_database, duckdb_disconnect,
    duckdb_register_table_function,
};
use std::ptr::null_mut;

pub struct Connection {
    ptr: duckdb_connection,
}

impl Connection {
    pub fn new(ptr: duckdb_database) -> Connection {
        let mut connection: duckdb_connection = null_mut();
        unsafe {
            check!(duckdb_connect(ptr, &mut connection));
        }
        Self { ptr: connection }
    }
    pub fn register_table_function(&self, table_function: TableFunction) {
        unsafe {
            check!(duckdb_register_table_function(self.ptr, table_function.ptr));
        }
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        unsafe {
            duckdb_disconnect(&mut self.ptr);
        }
    }
}
