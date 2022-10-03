use crate::DuckDBType;
use deltalake::SchemaDataType;

pub fn map_type(p0: &SchemaDataType) -> DuckDBType {
    match p0 {
        SchemaDataType::primitive(name) => {
            if name == "string" {
                //: utf8
                DuckDBType::Varchar
            } else if name == "long" {
                // undocumented, i64?
                DuckDBType::Bigint
            } else if name == "integer" {
                //: i32
                DuckDBType::Integer
            } else if name == "short" {
                //: i16
                DuckDBType::Smallint
            } else if name == "byte" {
                //: i8
                DuckDBType::Tinyint
            } else if name == "float" {
                //: f32
                DuckDBType::Float
            } else if name == "double" {
                //: f64
                DuckDBType::Double
            } else if name == "boolean" {
                //: bool
                DuckDBType::Boolean
            } else if name == "binary" {
                //: a sequence of binary data
                DuckDBType::Varchar
            } else if name == "date" {
                //: A calendar date, represented as a year-month-day triple without a timezone
                DuckDBType::Date
            } else if name == "timestamp" {
                //: Microsecond precision timestamp without a timezone
                DuckDBType::TimestampMs
            } else {
                panic!("unsupported primitive: {}", name);
            }
        }
        _ => {
            panic!("unknown type");
        }
    }
}
