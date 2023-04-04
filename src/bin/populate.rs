use deltalake::action::Action;
use deltalake::arrow::array::Int32Array;
use deltalake::arrow::datatypes::Field;
use deltalake::arrow::record_batch::RecordBatch;
use deltalake::writer::DeltaWriter;
use deltalake::writer::RecordBatchWriter;
use deltalake::DeltaTable;
use deltalake::DeltaTransactionOptions;
use deltalake::Schema;
use std::collections::HashMap;
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let id_array = Int32Array::from(vec![1, 2, 3, 4, 5]);
    let schema = Schema::new(vec![Field::new("id", DataType::Int32, false)]);

    let batch = RecordBatch::try_new(Arc::new(schema), vec![Arc::new(id_array)]).unwrap();

    /*let day = date(2022, 10, 4);
    let df = DataFrame({
        'x': [1, 2, 3],
        'other': [True, False, True],
        'third': ["foo", "baz", "bar"],
        'd': [day, day, day]
    });*/
    let table = DeltaTable::try_from_uri("../path/to/table");
    let mut writer = RecordBatchWriter::for_table(table)?;
    writer.write(batch)?;
    let actions: Vec<Action> = writer
        .flush()?
        .iter()
        .map(|add| Action::add(add.into()))
        .collect();
    let mut transaction = table.create_transaction(Some(DeltaTransactionOptions::new(
        /*max_retry_attempts=*/ 3,
    )));
    transaction.add_actions(actions);
    async {
        transaction.commit(Some(DeltaOperation::Write {
            mode: SaveMode::Append,
            partitionBy: Some(table.get_metadata().partition_columns),
            predicate: None,
        }))
    }
}
