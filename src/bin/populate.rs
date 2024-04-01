use deltalake::{
    arrow::{
        array::{BooleanArray, Date32Array, Int64Array, StringArray},
        compute::kernels::cast_utils::Parser,
        datatypes::{DataType, Date32Type, Field, Schema},
        record_batch::RecordBatch,
    },
    kernel::{Action, DataType as SchemaDataType, PrimitiveType, StructField},
    operations::transaction::commit,
    protocol::{DeltaOperation, SaveMode},
    writer::{DeltaWriter, RecordBatchWriter},
    DeltaOps, DeltaTable,
    DeltaTableError::NotATable,
};
use lazy_static::lazy_static;
use log::{info, LevelFilter};
use std::ops::Deref;
use std::sync::Arc;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::formatted_builder()
        .filter_level(LevelFilter::Info)
        .init();

    let batch = create_record_batch();

    let table = obtain_table().await;

    info!("table version: {}", table.version());

    let partition_columns = vec!["x".to_string()];

    let mut writer = RecordBatchWriter::for_table(&table).expect("for_table");
    writer.write(batch).await.expect("write");
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
        None,
        None,
    )
    .await?;

    info!("done");

    Ok(())
}

async fn obtain_table() -> DeltaTable {
    let mut table = mk_ops().await.0;

    match table.load().await {
        Err(err) => match err {
            NotATable(msg) => {
                info!("NotATable, creating table: {:?}", msg);
                create_table().await
            }
            _ => {
                panic!("error: {:?}", err);
            }
        },
        Ok(_) => {
            info!("table loaded successfully");
            table
        }
    }
}

lazy_static! {
    static ref COLUMNS: Vec<Field> = vec![
        Field::new("x", DataType::Int64, false),
        Field::new("other", DataType::Boolean, false),
        Field::new("third", DataType::Utf8, false),
        Field::new("d", DataType::Date32, false),
    ];
}

fn create_record_batch() -> RecordBatch {
    let schema = Schema::new(COLUMNS.clone());

    let day = Date32Type::parse("2022-10-04");

    RecordBatch::try_new(
        Arc::new(schema),
        vec![
            Arc::new(Int64Array::from(vec![1, 2, 3])),
            Arc::new(BooleanArray::from(vec![true, false, true])),
            Arc::new(StringArray::from(vec!["foo", "baz", "bar"])),
            Arc::new(Date32Array::from(vec![day, day, day])),
        ],
    )
    .unwrap()
}

async fn mk_ops() -> DeltaOps {
    DeltaOps::try_from_uri("test/simple_table")
        .await
        .expect("try_from_uri")
}

async fn create_table() -> DeltaTable {
    mk_ops()
        .await
        .create()
        .with_table_name("my_table")
        .with_columns(
            COLUMNS
                .clone()
                .iter()
                .map(|f| StructField::new(f.name().to_string(), map_type(f.data_type()), false))
                .collect::<Vec<StructField>>(),
        )
        .await
        .expect("create table")
}

fn map_type(data_type: &DataType) -> SchemaDataType {
    SchemaDataType::Primitive(match data_type {
        DataType::Boolean => PrimitiveType::Boolean,
        DataType::Int64 => PrimitiveType::Long,
        DataType::Date32 => PrimitiveType::Date,
        DataType::Utf8 => PrimitiveType::String,
        _ => panic!("unsupported type: {:?}", data_type),
    })
}
