pub static STANDARD_VECTOR_SIZE: usize = 1024;

pub static FUNCTION_NAME: &str = "read_delta\0";
use crate::duckly::{DUCKDB_TYPE_DUCKDB_TYPE_BIGINT, DUCKDB_TYPE_DUCKDB_TYPE_VARCHAR};

pub enum DuckDBType {
    // Boolean = 1,
    // Tinyint = 2,
    // Smallint = 3,
    // Integer = 4,
    Bigint = DUCKDB_TYPE_DUCKDB_TYPE_BIGINT as isize,
    // Utinyint = 6,
    // Usmallint = 7,
    // Uinteger = 8,
    // Ubigint = 9,
    // Float = 10,
    // Double = 11,
    // Timestamp = 12,
    // Date = 13,
    // Time = 14,
    // Interval = 15,
    // Hugeint = 16,
    Varchar = DUCKDB_TYPE_DUCKDB_TYPE_VARCHAR as isize,
    // Blob = 18,
    // Decimal = 19,
    // TimestampS = 20,
    // TimestampMs = 21,
    // TimestampNs = 22,
    // Enum = 23,
    // List = 24,
    // Struct = 25,
    // Map = 26,
    // Uuid = 27,
    // Json = 28,
}
