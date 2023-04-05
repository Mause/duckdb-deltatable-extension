use deltalake::SchemaDataType;
use duckdb::vtab::LogicalTypeId;

/// Maps Deltalake types to DuckDB types
pub fn map_type(p0: &SchemaDataType) -> LogicalTypeId {
    match p0 {
        SchemaDataType::primitive(name) => {
            if name == "string" {
                //: utf8
                LogicalTypeId::Varchar
            } else if name == "long" {
                // undocumented, i64?
                LogicalTypeId::Bigint
            } else if name == "integer" {
                //: i32
                LogicalTypeId::Integer
            } else if name == "short" {
                //: i16
                LogicalTypeId::Smallint
            } else if name == "byte" {
                //: i8
                LogicalTypeId::Tinyint
            } else if name == "float" {
                //: f32
                LogicalTypeId::Float
            } else if name == "double" {
                //: f64
                LogicalTypeId::Double
            } else if name == "boolean" {
                //: bool
                LogicalTypeId::Boolean
            } else if name == "binary" {
                //: a sequence of binary data
                LogicalTypeId::Varchar
            } else if name == "date" {
                //: A calendar date, represented as a year-month-day triple without a timezone
                LogicalTypeId::Date
            } else if name == "timestamp" {
                //: Microsecond precision timestamp without a timezone
                LogicalTypeId::TimestampMs
            } else {
                panic!("unsupported primitive: {}", name);
            }
        }
        _ => {
            panic!("unknown type");
        }
    }
}
