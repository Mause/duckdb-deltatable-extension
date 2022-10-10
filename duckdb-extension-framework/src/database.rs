use crate::check;
use crate::duckly::{duckdb_connect, duckdb_connection, duckdb_database};
use crate::Connection;
use std::ffi::c_void;
use std::ptr::{addr_of, null_mut};

/// Equivalent of [`DatabaseData`](https://github.com/duckdb/duckdb/blob/50951241de3d9c06fac5719dcb907eb21163dcab/src/include/duckdb/main/capi_internal.hpp#L27), wraps `duckdb::DuckDB`
#[repr(C)]
struct Wrapper {
    instance: *const c_void,
}

pub struct Database {
    db: duckdb_database,
}

impl Database {
    pub fn from_cpp_duckdb(ptr: *mut c_void) -> Self {
        let wrap = Wrapper { instance: ptr };

        Self {
            db: addr_of!(wrap) as duckdb_database,
        }
    }

    /// # Safety
    pub unsafe fn connect(&self) -> Connection {
        let mut connection: duckdb_connection = null_mut();
        check!(duckdb_connect(self.db, &mut connection));
        Connection::from(connection)
    }
}
