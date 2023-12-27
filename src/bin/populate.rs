use deltalake::{
    arrow::{
        array::{
            Array, BooleanArray, Date32Array, Int32Builder, Int64Array, ListBuilder, StringArray,
        },
        compute::kernels::cast_utils::Parser,
        datatypes::{DataType, Date32Type, Field, Schema},
        record_batch::RecordBatch,
    },
    operations::transaction::commit,
    protocol::{Action, DeltaOperation, SaveMode},
    writer::{DeltaWriter, RecordBatchWriter},
    DeltaOps, DeltaTable,
    DeltaTableError::NotATable,
    SchemaDataType, SchemaField, SchemaTypeArray, SchemaTypeMap, SchemaTypeStruct,
};
use log::{info, LevelFilter};
use std::{collections::HashMap, env::args, ops::Deref};
use std::{slice::Iter, sync::Arc};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::formatted_builder()
        .filter_level(LevelFilter::Info)
        .init();

    args().for_each(|arg| {
        info!("arg: {}", arg);
    });

    let mut with_list = false;
    for arg in args().skip(1) {
        if arg == "--with-list" {
            with_list = true;
            info!("with_list: {}", arg);
        } else {
            panic!("unknown arg: {}", arg);
        }
    }

    let batch = create_record_batch(with_list);

    let table = obtain_table(with_list).await;

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

async fn obtain_table(with_list: bool) -> DeltaTable {
    let mut table = mk_ops().await.0;

    match table.load().await {
        Err(err) => match err {
            NotATable(msg) => {
                info!("NotATable, creating table: {:?}", msg);
                create_table(with_list).await
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

fn get_columns(with_list: bool) -> Vec<Field> {
    let mut vector = vec![
        Field::new("x", DataType::Int64, false),
        Field::new("other", DataType::Boolean, false),
        Field::new("third", DataType::Utf8, false),
        Field::new("d", DataType::Date32, false),
        // Field::new(
        //     "structed",
        //     DataType::Struct(
        //         vec![
        //             Field::new("a", DataType::Int64, false),
        //             Field::new("b", DataType::Utf8, false),
        //         ]
        //         .into()
        //     ),
        //     false
        // ),
    ];
    if with_list {
        vector.append(&mut vec![Field::new(
            "listed",
            DataType::List(Arc::new(Field::new("item", DataType::Int32, true))),
            true,
        )]);
    }

    vector
}

fn create_record_batch(with_list: bool) -> RecordBatch {
    let schema = Schema::new(get_columns(with_list));

    let day = Date32Type::parse("2022-10-04");

    let mut lst = ListBuilder::new(Int32Builder::new());
    lst.values().append_value(1);
    lst.append(true);
    lst.values().append_value(2);
    lst.append(true);
    lst.values().append_value(3);
    lst.append(true);
    let end_list = lst.finish();

    assert_eq!(
        end_list.data_type(),
        schema.fields().last().unwrap().data_type()
    );

    RecordBatch::try_new(
        Arc::new(schema),
        vec![
            Arc::new(Int64Array::from(vec![1, 2, 3])),
            Arc::new(BooleanArray::from(vec![true, false, true])),
            Arc::new(StringArray::from(vec!["foo", "baz", "bar"])),
            Arc::new(Date32Array::from(vec![day, day, day])),
            Arc::new(end_list),
        ],
    )
    .unwrap()
}

async fn mk_ops() -> DeltaOps {
    DeltaOps::try_from_uri("test/simple_table")
        .await
        .expect("try_from_uri")
}

async fn create_table(with_list: bool) -> DeltaTable {
    mk_ops()
        .await
        .create()
        .with_table_name("my_table")
        .with_columns(
            get_columns(with_list)
                .iter()
                .map(|f| {
                    SchemaField::new(
                        f.name().to_string(),
                        map_type(f.data_type()),
                        f.is_nullable(),
                        HashMap::new(),
                    )
                })
                .collect::<Vec<SchemaField>>(),
        )
        .await
        .expect("create table")
}

fn map_type(data_type: &DataType) -> SchemaDataType {
    if data_type.is_primitive()
        || matches!(data_type, DataType::Boolean)
        || matches!(data_type, DataType::Utf8)
    {
        SchemaDataType::primitive(
            match data_type {
                DataType::Boolean => "boolean",
                DataType::Int64 => "long",
                DataType::Int32 => "integer",
                DataType::Date32 => "date",
                DataType::Utf8 => "string",
                _ => panic!("unsupported primitive type: {:?}", data_type),
            }
            .to_string(),
        )
    } else if data_type.is_nested() {
        match data_type {
            DataType::Struct(fields) => {
                SchemaDataType::r#struct(SchemaTypeStruct::new(create_schema_fields(fields.iter())))
            }
            DataType::Map(field_type, _sortable) => {
                let key_type = map_type(field_type.data_type());
                let value_type = map_type(field_type.data_type());

                SchemaDataType::map(SchemaTypeMap::new(
                    Box::new(key_type),
                    Box::new(value_type),
                    field_type.is_nullable(),
                ))
            }
            DataType::List(element_type) => SchemaDataType::array(SchemaTypeArray::new(
                Box::new(map_type(element_type.data_type())),
                element_type.is_nullable(),
            )),
            _ => panic!("unsupported nested type: {:?}", data_type),
        }
    } else {
        panic!(
            "unsupported type (not nested or primitive?): {:?}",
            data_type
        )
    }
}

fn create_schema_fields(fields: Iter<Arc<Field>>) -> Vec<SchemaField> {
    fields
        .map(|a| {
            SchemaField::new(
                a.name().to_string(),
                map_type(a.data_type()),
                a.is_nullable(),
                HashMap::new(),
            )
        })
        .collect()
}
