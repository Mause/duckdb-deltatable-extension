use deltalake::{
    arrow::{
        array::{BooleanArray, Date32Array, Int64Array, StringArray},
        compute::kernels::cast_utils::Parser,
        datatypes::{DataType, Date32Type, Field, Schema},
        record_batch::RecordBatch,
    },
    operations::transaction::commit,
    protocol::{Action, DeltaOperation, SaveMode},
    writer::{DeltaWriter, RecordBatchWriter},
    DeltaOps, DeltaTable,
    DeltaTableError::NotATable,
    SchemaDataType, SchemaField,
};
use lazy_static::lazy_static;
use log::{info, LevelFilter};
use std::sync::Arc;
use std::{collections::HashMap, ops::Deref};

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
        .map(|add| Action::add(add.clone()))
        .collect();

    commit(
        table.object_store().deref(),
        &actions,
        DeltaOperation::Write {
            mode: SaveMode::Append,
            partition_by: Some(partition_columns.clone()),
            predicate: None,
        },
        table.get_state(),
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
                .map(|f| {
                    SchemaField::new(
                        f.name().to_string(),
                        map_type(f.data_type()),
                        false,
                        HashMap::new(),
                    )
                })
                .collect::<Vec<SchemaField>>(),
        )
        .await
        .expect("create table")
}

fn map_type(data_type: &DataType) -> SchemaDataType {
    SchemaDataType::primitive(
        match data_type {
            DataType::Boolean => "boolean",
            DataType::Int64 => "long",
            DataType::Date32 => "date",
            DataType::Utf8 => "string",
            _ => panic!("unsupported type: {:?}", data_type),
        }
        .to_string(),
    )
}
