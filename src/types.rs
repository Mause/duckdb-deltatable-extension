use deltalake::SchemaDataType;
use duckdb::vtab::{LogicalType, LogicalTypeId};

/// Maps Deltalake types to DuckDB types
pub fn map_type(p0: &SchemaDataType) -> LogicalType {
    match p0 {
        SchemaDataType::primitive(name) => LogicalType::new(map_primitive_type(name)),
        SchemaDataType::array(p0) => {
            //: a sequence of elements, all with the same type
            LogicalType::list(&map_type(p0.get_element_type()))
        }
        SchemaDataType::map(p0) => {
            //: a sequence of key-value pairs, with a single key type, and a single value type
            LogicalType::map(&map_type(p0.get_key_type()), &map_type(p0.get_value_type()))
        }
        _ => {
            panic!("unknown type");
        }
    }
}

fn map_primitive_type(name: &String) -> LogicalTypeId {
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
