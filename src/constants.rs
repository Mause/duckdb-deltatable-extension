pub static STANDARD_VECTOR_SIZE: usize = 1024;

pub static FUNCTION_NAME: &str = "read_delta\0";
use crate::duckly::*;

pub enum DuckDBType {
    Boolean = DUCKDB_TYPE_DUCKDB_TYPE_BOOLEAN as isize,
    Tinyint = DUCKDB_TYPE_DUCKDB_TYPE_TINYINT as isize,
    Smallint = DUCKDB_TYPE_DUCKDB_TYPE_SMALLINT as isize,
    Integer = DUCKDB_TYPE_DUCKDB_TYPE_INTEGER as isize,
    Bigint = DUCKDB_TYPE_DUCKDB_TYPE_BIGINT as isize,
    // Utinyint = 6,
    // Usmallint = 7,
    // Uinteger = 8,
    // Ubigint = 9,
    Float = DUCKDB_TYPE_DUCKDB_TYPE_FLOAT as isize,
    Double = DUCKDB_TYPE_DUCKDB_TYPE_DOUBLE as isize,
    // Timestamp = 12,
    Date = DUCKDB_TYPE_DUCKDB_TYPE_DATE as isize,
    // Time = 14,
    // Interval = 15,
    // Hugeint = 16,
    Varchar = DUCKDB_TYPE_DUCKDB_TYPE_VARCHAR as isize,
    // Blob = 18,
    Decimal = DUCKDB_TYPE_DUCKDB_TYPE_DECIMAL as isize,
    // TimestampS = 20,
    TimestampMs = DUCKDB_TYPE_DUCKDB_TYPE_TIMESTAMP_MS as isize,
    // TimestampNs = 22,
    // Enum = 23,
    // List = 24,
    // Struct = 25,
    // Map = 26,
    // Uuid = 27,
    // Json = 28,
}
