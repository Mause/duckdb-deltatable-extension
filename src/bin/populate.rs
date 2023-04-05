use deltalake::arrow::compute::kernels::cast_utils::Parser;
use deltalake::arrow::datatypes::Date32Type;
use deltalake::DeltaTableError::NotATable;
use deltalake::{
    action::{Action, DeltaOperation, SaveMode},
    arrow::{
        array::{BooleanArray, Date32Array, Int32Array, StringArray},
        datatypes::{DataType, Field, Schema},
        record_batch::RecordBatch,
    },
    writer::{DeltaWriter, RecordBatchWriter},
    DeltaOps, DeltaTable, SchemaDataType,
};
use log::{info, LevelFilter};
use std::sync::Arc;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::formatted_builder()
        .filter_level(LevelFilter::Info)
        .init();

    let batch = create_record_batch();

    let mut table = obtain_table().await;

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
    let mut transaction = table.create_transaction(None);
    transaction.add_actions(actions);
    transaction
        .commit(
            Some(DeltaOperation::Write {
                mode: SaveMode::Append,
                partition_by: Some(partition_columns.clone()),
                predicate: None,
            }),
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

fn create_record_batch() -> RecordBatch {
    let schema = Schema::new(vec![
        Field::new("x", DataType::Int32, false),
        Field::new("other", DataType::Boolean, false),
        Field::new("third", DataType::Utf8, false),
        Field::new("d", DataType::Date32, false),
    ]);

    let day = Date32Type::parse("2022-10-04");

    RecordBatch::try_new(
        Arc::new(schema),
        vec![
            Arc::new(Int32Array::from(vec![1, 2, 3])),
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
        .with_column(
            "x".to_string(),
            SchemaDataType::primitive("integer".to_string()),
            false,
            None,
        )
        .with_column(
            "other".to_string(),
            SchemaDataType::primitive("boolean".to_string()),
            false,
            None,
        )
        .with_column(
            "third".to_string(),
            SchemaDataType::primitive("string".to_string()),
            false,
            None,
        )
        .with_column(
            "d".to_string(),
            SchemaDataType::primitive("date".to_string()),
            false,
            None,
        )
        .await
        .expect("create table")
}
