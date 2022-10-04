#![allow(dead_code)]
use crate::constants::{DuckDBType, FUNCTION_NAME};
use std::os::raw::c_char;
use std::ptr::{addr_of, addr_of_mut, null, null_mut};

use crate::duckly::{
    duckdb_close, duckdb_connect, duckdb_connection, duckdb_data_chunk_get_vector, duckdb_database,
    duckdb_destroy_data_chunk, duckdb_destroy_logical_type, duckdb_destroy_result,
    duckdb_destroy_table_function, duckdb_disconnect, duckdb_get_type_id, duckdb_open,
    duckdb_query, duckdb_register_table_function, duckdb_result, duckdb_result_get_chunk,
    duckdb_vector_get_column_type, duckdb_vector_get_data,
};
use crate::table_function::build_table_function_def;

mod constants;
mod duckly;
mod error;
mod strings;
mod table_function;
mod types;

/// Equivalent of [`DatabaseData`](https://github.com/duckdb/duckdb/blob/50951241de3d9c06fac5719dcb907eb21163dcab/src/include/duckdb/main/capi_internal.hpp#L27), wraps `duckdb::DuckDB`
#[repr(C)]
struct Wrapper {
    instance: *const u8,
}

/// Init hook for DuckDB, registers all functionality provided by this extension
#[no_mangle]
pub extern "C" fn libtest_extension_init_rust(db: *mut u8) {
    unsafe {
        let wrap = Wrapper { instance: db };

        let real_db = addr_of!(wrap) as duckdb_database;

        let mut table_function = build_table_function_def();

        let mut connection: duckdb_connection = null_mut();
        check!(duckdb_connect(real_db, &mut connection));
        check!(duckdb_register_table_function(connection, table_function));
        duckdb_disconnect(&mut connection);

        duckdb_destroy_table_function(&mut table_function);
    }
}

/// Version hook for DuckDB, indicates which version of DuckDB this extension was compiled against
#[no_mangle]
pub extern "C" fn libtest_extension_version_rust() -> *const c_char {
    unsafe {
        let mut database: duckdb_database = null_mut();
        let mut connection: duckdb_connection = null_mut();
        let mut result = duckdb_result::default();

        check!(duckdb_open(null(), &mut database));
        check!(duckdb_connect(database, &mut connection));
        check!(duckdb_query(
            connection,
            as_string!("pragma version"),
            addr_of_mut!(result),
        ));
        let mut chunk = duckdb_result_get_chunk(result, 0);
        let vect = duckdb_data_chunk_get_vector(chunk, 0);

        let mut column_type = duckdb_vector_get_column_type(vect);
        assert_eq!(duckdb_get_type_id(column_type), DuckDBType::Varchar as u32);
        duckdb_destroy_logical_type(addr_of_mut!(column_type));

        let data = duckdb_vector_get_data(vect);

        let res = strings::convert_string(data, 1);

        duckdb_destroy_data_chunk(&mut chunk);
        duckdb_destroy_result(&mut result);

        duckdb_disconnect(&mut connection);
        duckdb_close(&mut database);

        strings::static_version_string(res)
    }
}
