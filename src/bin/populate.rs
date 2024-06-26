use argparse_rs::{ArgParser, ArgType};
use deltalake::{
    arrow::{
        array::{
            Array, ArrayRef, BooleanArray, Date32Array, Decimal128Builder, Int32Builder,
            Int64Array, ListBuilder, StringArray, StructArray,
        },
        compute::kernels::cast_utils::Parser,
        datatypes::{DataType, Date32Type, Field, Schema},
        record_batch::RecordBatch,
    },
    kernel::{
        Action, ArrayType, DataType as SchemaDataType, MapType, PrimitiveType, Protocol,
        StructField, StructType,
    },
    operations::transaction::commit,
    protocol::{DeltaOperation, SaveMode},
    writer::{DeltaWriter, RecordBatchWriter},
    DeltaOps, DeltaTable,
    DeltaTableError::{self, NotATable},
};
use log::{info, LevelFilter};
use std::{env::args, ops::Deref};
use std::{slice::Iter, sync::Arc};

struct Config {
    filename: String,
    with_list: bool,
    with_struct: bool,
    with_decimal: bool,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::formatted_builder()
        .filter_level(LevelFilter::Info)
        .init();

    let mut parser = ArgParser::new("Populate".to_string());
    parser.add_opt(
        "filename",
        None,
        'f',
        true,
        "Filename to write table to",
        ArgType::Positional(0),
    );
    parser.add_opt(
        "with-list",
        None,
        'l',
        false,
        "with list will include a list column",
        ArgType::Flag,
    );
    parser.add_opt(
        "with-struct",
        None,
        's',
        false,
        "with struct will include a struct column",
        ArgType::Flag,
    );
    parser.add_opt(
        "with-decimal",
        None,
        'd',
        false,
        "with decimal will include a decimal column",
        ArgType::Flag,
    );
    let parsed = parser.parse(args().collect::<Vec<String>>().iter())?;

    let get = |g: &str| {
        parsed
            .get_with(g, |user: &str| Some(user == "true"))
            .unwrap_or(false)
    };

    if get("help") {
        println!("Populate a delta table with some data");
        parser.help();
        return Ok(());
    }
    let config = Config {
        filename: parsed.get("filename").expect("missing filename"),
        with_list: get("with-list"),
        with_struct: get("with-struct"),
        with_decimal: get("with-decimal"),
    };

    let batch = create_record_batch(&config);

    let table = obtain_table(&config).await;

    info!("table version: {}", table.version());

    let partition_columns = vec!["x".to_string()];

    let mut writer = RecordBatchWriter::for_table(&table).expect("for_table");
    if let Err(err) = writer.write(batch).await {
        match err {
            DeltaTableError::Generic(msg) => {
                if msg.contains("schema does not match") {
                    panic!("schema does not match: clean and try again");
                }
            }
            _ => {
                panic!("error: {:?}", err);
            }
        }
    }

    let actions: Vec<Action> = writer
        .flush()
        .await?
        .iter()
        .map(|add| Action::Add(add.clone()))
        .collect();

    commit(
        table.log_store().deref(),
        &actions,
        DeltaOperation::Write {
            mode: SaveMode::Append,
            partition_by: Some(partition_columns.clone()),
            predicate: None,
        },
        Some(&table.state.expect("no state")),
        None,
    )
    .await
    .expect("commit failed");

    info!("done");

    Ok(())
}

async fn obtain_table(config: &Config) -> DeltaTable {
    let mut table = mk_ops(config.filename.as_str()).await.0;

    match table.load().await {
        Err(err) => match err {
            NotATable(msg) => {
                info!("NotATable, creating table: {:?}", msg);
                create_table(config).await
            }
            _ => {
                panic!("error: {:?}", err);
            }
        },
        Ok(_) => {
            assert_eq!(table.protocol().unwrap().min_writer_version, 0);
            info!("table loaded successfully");
            table
        }
    }
}

fn get_columns(config: &Config) -> Vec<Field> {
    let mut vector = vec![
        Field::new("x", DataType::Int64, false),
        Field::new("other", DataType::Boolean, false),
        Field::new("third", DataType::Utf8, false),
        Field::new("d", DataType::Date32, false),
    ];

    if config.with_struct {
        vector.push(Field::new(
            "structed",
            DataType::Struct(
                vec![
                    Field::new("a", DataType::Int64, false),
                    Field::new("b", DataType::Utf8, false),
                ]
                .into(),
            ),
            false,
        ));
    }
    if config.with_list {
        vector.push(Field::new(
            "listed",
            DataType::List(Arc::new(Field::new("item", DataType::Int32, true))),
            true,
        ));
    }
    if config.with_decimal {
        vector.push(Field::new("decimal", DataType::Decimal128(10, 2), false));
    }

    vector
}

fn create_record_batch(config: &Config) -> RecordBatch {
    let schema = Schema::new(get_columns(config));

    let day = Date32Type::parse("2022-10-04");

    let mut lst = ListBuilder::new(Int32Builder::new());
    lst.values().append_values(&[1, 2, 3], &[true, true, true]);
    lst.append(true);
    lst.values().append_value(2);
    lst.append(true);
    lst.values().append_value(3);
    lst.append(true);
    let end_list = lst.finish();
    assert_eq!(end_list.len(), 3);

    if config.with_list {
        assert_eq!(
            end_list.data_type(),
            schema.fields().last().unwrap().data_type()
        );
    }

    let struct_a = Int64Array::from(vec![1, 2, 3]);
    let struct_b = StringArray::from(vec!["foo", "baz", "bar"]);

    let mut columns: Vec<ArrayRef> = vec![
        Arc::new(Int64Array::from(vec![1, 2, 3])),
        Arc::new(BooleanArray::from(vec![true, false, true])),
        Arc::new(StringArray::from(vec!["foo", "baz", "bar"])),
        Arc::new(Date32Array::from(vec![day, day, day])),
    ];

    if config.with_list {
        columns.push(Arc::new(end_list));
    }
    if config.with_struct {
        columns.push(Arc::new(StructArray::from(vec![
            (
                Arc::new(Field::new("a", DataType::Int64, false)),
                Arc::new(struct_a) as ArrayRef,
            ),
            (
                Arc::new(Field::new("b", DataType::Utf8, false)),
                Arc::new(struct_b) as ArrayRef,
            ),
        ])));
    };
    if config.with_decimal {
        let mut builder = Decimal128Builder::new()
            .with_precision_and_scale(10, 2)
            .expect("with_precision_and_scale");
        builder.append_value(1);
        builder.append_value(2);
        builder.append_value(3);
        let decimal = builder.finish();
        columns.push(Arc::new(decimal));
    }

    RecordBatch::try_new(Arc::new(schema), columns).unwrap()
}

async fn mk_ops(filename: &str) -> DeltaOps {
    DeltaOps::try_from_uri(filename)
        .await
        .expect("try_from_uri")
}

async fn create_table(config: &Config) -> DeltaTable {
    mk_ops(config.filename.as_str())
        .await
        .create()
        .with_table_name("my_table")
        .with_actions([Action::Protocol(Protocol::new(0, 0))])
        .with_columns(
            get_columns(config)
                .iter()
                .map(|f| {
                    StructField::new(
                        f.name().to_string(),
                        map_type(f.data_type()),
                        f.is_nullable(),
                    )
                })
                .collect::<Vec<StructField>>(),
        )
        .await
        .expect("create table")
}

fn map_type(data_type: &DataType) -> SchemaDataType {
    if data_type.is_primitive()
        || matches!(data_type, DataType::Boolean)
        || matches!(data_type, DataType::Utf8)
    {
        SchemaDataType::Primitive(match data_type {
            DataType::Boolean => PrimitiveType::Boolean,
            DataType::Int64 => PrimitiveType::Long,
            DataType::Int32 => PrimitiveType::Integer,
            DataType::Date32 => PrimitiveType::Date,
            DataType::Utf8 => PrimitiveType::String,
            DataType::Decimal128(precision, scale) => PrimitiveType::Decimal(*precision, *scale),
            _ => todo!("unsupported primitive type: {:?}", data_type),
        })
    } else if data_type.is_nested() {
        match data_type {
            DataType::Struct(fields) => SchemaDataType::Struct(Box::new(StructType::new(
                create_schema_fields(fields.iter()),
            ))),
            DataType::Map(field_type, _sortable) => {
                let key_type = map_type(field_type.data_type());
                let value_type = map_type(field_type.data_type());

                SchemaDataType::Map(Box::new(MapType::new(
                    key_type,
                    value_type,
                    field_type.is_nullable(),
                )))
            }
            DataType::List(element_type) => SchemaDataType::Array(Box::new(ArrayType::new(
                map_type(element_type.data_type()),
                element_type.is_nullable(),
            ))),
            _ => panic!("unsupported nested type: {:?}", data_type),
        }
    } else {
        panic!(
            "unsupported type (not nested or primitive?): {:?}",
            data_type
        )
    }
}

fn create_schema_fields(fields: Iter<Arc<Field>>) -> Vec<StructField> {
    fields
        .map(|a| {
            StructField::new(
                a.name().to_string(),
                map_type(a.data_type()),
                a.is_nullable(),
            )
        })
        .collect()
}
