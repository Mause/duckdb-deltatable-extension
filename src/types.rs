use deltalake::kernel::{DataType as SchemaDataType, PrimitiveType};
use duckdb::vtab::LogicalTypeId;

/// Maps Deltalake types to DuckDB types
pub fn map_type(p0: &SchemaDataType) -> LogicalTypeId {
    match p0 {
        SchemaDataType::Primitive(name) => match name {
            PrimitiveType::String => {
                //: utf8
                LogicalTypeId::Varchar
            }
            PrimitiveType::Long => {
                // undocumented, i64?
                LogicalTypeId::Bigint
            }
            PrimitiveType::Integer => {
                //: i32
                LogicalTypeId::Integer
            }
            PrimitiveType::Short => {
                //: i16
                LogicalTypeId::Smallint
            }
            PrimitiveType::Byte => {
                //: i8
                LogicalTypeId::Tinyint
            }
            PrimitiveType::Float => {
                //: f32
                LogicalTypeId::Float
            }
            PrimitiveType::Double => {
                //: f64
                LogicalTypeId::Double
            }
            PrimitiveType::Boolean => {
                //: bool
                LogicalTypeId::Boolean
            }
            PrimitiveType::Binary => {
                //: a sequence of binary data
                LogicalTypeId::Varchar
            }
            PrimitiveType::Date => {
                //: A calendar date, represented as a year-month-day triple without a timezone
                LogicalTypeId::Date
            }
            PrimitiveType::Timestamp => {
                //: Microsecond precision timestamp without a timezone
                LogicalTypeId::TimestampMs
            }
            _ => panic!("unsupported primitive: {}", name),
        },
        _ => {
            panic!("unknown type");
        }
    }
}
