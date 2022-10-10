use crate::duckly::{duckdb_connection, duckdb_disconnect, duckdb_register_table_function};
use crate::{check, TableFunction};

pub struct Connection {
    ptr: duckdb_connection,
}

impl From<duckdb_connection> for Connection {
    fn from(ptr: duckdb_connection) -> Self {
        Self { ptr }
    }
}

impl Connection {
    pub fn register_table_function(
        &self,
        table_function: TableFunction,
    ) -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            check!(duckdb_register_table_function(self.ptr, table_function.ptr));
        }
        Ok(())
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        unsafe {
            duckdb_disconnect(&mut self.ptr);
        }
    }
}
