use crate::DuckDBType;
use deltalake::SchemaDataType;

pub fn map_type(p0: &SchemaDataType) -> DuckDBType {
    match p0 {
        SchemaDataType::primitive(name) => {
            if name == "string" {
                DuckDBType::Varchar
            }
            //: utf8
            else if name == "long" {
                DuckDBType::Bigint
            }
            //  // undocumented, i64?
            else if name == "integer" {
                DuckDBType::Integer
            }
            //: i32
            else if name == "short" {
                DuckDBType::Smallint
            }
            //: i16
            else if name == "byte" {
                DuckDBType::Tinyint
            }
            //: i8
            else if name == "float" {
                DuckDBType::Float
            }
            //: f32
            else if name == "double" {
                DuckDBType::Double
            }
            //: f64
            else if name == "boolean" {
                DuckDBType::Boolean
            }
            //: bool
            else if name == "binary" {
                DuckDBType::Varchar
            }
            //: a sequence of binary data
            else if name == "date" {
                DuckDBType::Date
            }
            //: A calendar date, represented as a year-month-day triple without a timezone
            else if name == "timestamp" {
                DuckDBType::TimestampMs
            }
            //: Microsecond precision timestamp without a timezone
            else {
                panic!("unsupported primitive: {}", name);
            }
        }
        _ => {
            panic!("unknown type");
        }
    }
}
