use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use deltalake::{
    action::{Action, DeltaOperation, SaveMode},
    arrow::{
        array::{BooleanArray, Date64Array, Int32Array, StringArray},
        datatypes::{DataType, Field, Schema},
        record_batch::RecordBatch,
    },
    writer::{DeltaWriter, RecordBatchWriter},
    DeltaOps, SchemaDataType,
};
use std::sync::Arc;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schema = Schema::new(vec![
        Field::new("x", DataType::Int32, false),
        Field::new("other", DataType::Boolean, false),
        Field::new("third", DataType::Utf8, false),
        Field::new("d", DataType::Date64, false),
    ]);

    let day = NaiveDateTime::new(
        "2022-10-04".parse::<NaiveDate>()?,
        NaiveTime::from_num_seconds_from_midnight_opt(0, 0).unwrap(),
    )
    .timestamp_millis();

    let batch = RecordBatch::try_new(
        Arc::new(schema),
        vec![
            Arc::new(Int32Array::from(vec![1, 2, 3])),
            Arc::new(BooleanArray::from(vec![true, false, true])),
            Arc::new(StringArray::from(vec!["foo", "baz", "bar"])),
            Arc::new(Date64Array::from(vec![day, day, day])),
        ],
    )
    .unwrap();

    let ops = DeltaOps::try_from_uri("test/simple_table")
        .await
        .expect("try_from_uri");

    // if let Ok(table) = ops.0.load().await {

    // }

    let mut table = ops
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
        .expect("create table");
    assert_eq!(table.version(), 0);

    let partition_columns = vec!["x".to_string()];

    let mut writer = RecordBatchWriter::for_table(&table).expect("for_table");
    writer.write(batch).await?;
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

    Ok(())
}
