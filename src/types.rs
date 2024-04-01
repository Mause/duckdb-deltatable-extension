use deltalake::SchemaDataType;
use duckdb::vtab::LogicalTypeId;

/// Maps Deltalake types to DuckDB types
pub fn map_type(p0: &SchemaDataType) -> LogicalTypeId {
    match p0 {
        SchemaDataType::primitive(name) => match name.as_str() {
            "string" => {
                //: utf8
                LogicalTypeId::Varchar
            }
            "long" => {
                // undocumented, i64?
                LogicalTypeId::Bigint
            }
            "integer" => {
                //: i32
                LogicalTypeId::Integer
            }
            "short" => {
                //: i16
                LogicalTypeId::Smallint
            }
            "byte" => {
                //: i8
                LogicalTypeId::Tinyint
            }
            "float" => {
                //: f32
                LogicalTypeId::Float
            }
            "double" => {
                //: f64
                LogicalTypeId::Double
            }
            "boolean" => {
                //: bool
                LogicalTypeId::Boolean
            }
            "binary" => {
                //: a sequence of binary data
                LogicalTypeId::Varchar
            }
            "date" => {
                //: A calendar date, represented as a year-month-day triple without a timezone
                LogicalTypeId::Date
            }
            "timestamp" => {
                //: Microsecond precision timestamp without a timezone
                LogicalTypeId::TimestampMs
            }
            _ => {
                panic!("unsupported primitive: {}", name);
            }
        },
        _ => {
            panic!("unknown type");
        }
    }
}
