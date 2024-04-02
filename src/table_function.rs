use deltalake::open_table;
use duckdb::ffi::duckdb_vector_size;
use duckdb::vtab::{
    BindInfo, DataChunk, Free, FunctionInfo, InitInfo, Inserter, LogicalType, LogicalTypeId, VTab,
};
use parquet::data_type::AsBytes;
use std::error::Error;
use std::ffi::{CStr, CString};
use std::fs::File;
use std::os::raw::c_char;
use std::path::Path;
use tokio::runtime::Runtime;

use crate::types::map_type;
use parquet::file::reader::SerializedFileReader;
use parquet::record::Field;

#[repr(C)]
pub struct MyBindDataStruct {
    filename: *mut c_char,
}
impl Free for MyBindDataStruct {
    fn free(&mut self) {
        unsafe {
            drop(CString::from_raw(self.filename.cast()));
        }
    }
}

#[repr(C)]
pub struct MyInitDataStruct {
    done: bool,
    file_idx: usize,
    spent: usize,
}
impl Free for MyInitDataStruct {}

/// # Safety
///
/// .
fn read_delta(info: &FunctionInfo, output: &mut DataChunk) {
    let bind_data = info.get_bind_data::<MyBindDataStruct>();
    let init_data = info.get_init_data::<MyInitDataStruct>();

    let filename = unsafe { CStr::from_ptr((*bind_data).filename) };

    let table_result = RUNTIME.block_on(open_table(filename.to_str().unwrap()));

    if let Err(err) = table_result {
        info.set_error(&err.to_string());
        return;
    }

    let table = table_result.unwrap();

    let root_dir = Path::new(filename.to_str().unwrap());
    let mut row_idx: usize = 0;
    for (file_idx, pq_filename) in table.get_files_iter().unwrap().enumerate() {
        unsafe {
            (*init_data).file_idx = file_idx;
            if (*init_data).done {
                break;
            }
        }
        let reader =
            SerializedFileReader::new(File::open(root_dir.join(pq_filename.to_string())).unwrap())
                .unwrap();

        for row in reader {
            for (col_idx, (_key, value)) in row.expect("missing row?").get_column_iter().enumerate()
            {
                populate_column(value, output, row_idx, col_idx);
            }
            row_idx += 1;

            unsafe {
                assert!(
                    row_idx < duckdb_vector_size().try_into().unwrap(),
                    "overflowed vector: {}",
                    row_idx
                );
            }
        }
    }
    unsafe {
        (*init_data).spent += row_idx;
    }
    output.set_len(row_idx);
}

fn populate_column(value: &Field, output: &DataChunk, row_idx: usize, col_idx: usize) {
    match value {
        Field::Int(v) => {
            assign(output, row_idx, col_idx, *v);
        }
        Field::Bool(v) => {
            assign(output, row_idx, col_idx, *v);
        }
        Field::Long(v) => {
            assign(output, row_idx, col_idx, *v);
        }
        Field::Date(v) => {
            assign(output, row_idx, col_idx, *v);
        }
        Field::Float(v) => {
            assign(output, row_idx, col_idx, *v);
        }
        Field::Byte(v) => {
            assign(output, row_idx, col_idx, *v);
        }
        Field::Short(v) => {
            assign(output, row_idx, col_idx, *v);
        }
        Field::UByte(v) => {
            assign(output, row_idx, col_idx, *v);
        }
        Field::UShort(v) => {
            assign(output, row_idx, col_idx, *v);
        }
        Field::UInt(v) => {
            assign(output, row_idx, col_idx, *v);
        }
        Field::ULong(v) => {
            assign(output, row_idx, col_idx, *v);
        }
        Field::Double(v) => {
            assign(output, row_idx, col_idx, *v);
        }
        // Field::Decimal(v) => {
        //     assign(&output, row_row, idx, duckdb_double_to_hugeint(*v));
        // },
        Field::TimestampMillis(v) => {
            assign(output, row_idx, col_idx, *v);
        }
        Field::TimestampMicros(v) => {
            assign(output, row_idx, col_idx, *v);
        }
        Field::Bytes(v) => {
            set_bytes(output, row_idx, col_idx, v.as_bytes());
        }
        Field::Str(v) => {
            set_bytes(output, row_idx, col_idx, v.as_bytes());
        }
        // TODO: support more types
        _ => todo!("{}", value),
    }
}

fn set_bytes(output: &DataChunk, row_idx: usize, col_idx: usize, bytes: &[u8]) {
    let cs = CString::new(bytes).unwrap();

    let result_vector = output.flat_vector(col_idx);

    assert_eq!(result_vector.logical_type().id(), LogicalTypeId::Varchar);

    result_vector.insert(row_idx, cs);
}

fn assign<T>(output: &DataChunk, row_idx: usize, col_idx: usize, v: T) {
    output.flat_vector(col_idx).as_mut_slice::<T>()[row_idx] = v;
}

/// # Safety
///
/// .
fn read_delta_bind(bind_info: &BindInfo, my_bind_data: *mut MyBindDataStruct) {
    assert_eq!(bind_info.get_parameter_count(), 1);

    let string = bind_info.get_parameter(0).to_string();
    let filename = string.as_str();

    let handle = RUNTIME.block_on(open_table(filename));
    if let Err(err) = handle {
        bind_info.set_error(&err.to_string());
        return;
    }

    let table = handle.unwrap();
    let schema = table.schema().expect("no schema");
    for field in schema.fields() {
        let typ = LogicalType::new(map_type(field.data_type()));
        bind_info.add_result_column(field.name(), typ);
    }

    unsafe {
        (*my_bind_data).filename = CString::new(filename).expect("c string").into_raw();
    }
}

lazy_static::lazy_static! {
    static ref RUNTIME: Runtime = tokio::runtime::Builder::new_current_thread()
            .build()
            .expect("runtime");
}

pub struct DeltaFunction {}
impl VTab for DeltaFunction {
    type InitData = MyInitDataStruct;
    type BindData = MyBindDataStruct;

    fn bind(bind: &BindInfo, data: *mut Self::BindData) -> duckdb::Result<(), Box<dyn Error>> {
        read_delta_bind(bind, data);

        Ok(())
    }

    fn init(_init: &InitInfo, data: *mut Self::InitData) -> duckdb::Result<(), Box<dyn Error>> {
        unsafe {
            (*data).done = true;
            (*data).file_idx = 0;
            (*data).spent = 0;
        }

        Ok(())
    }

    fn func(func: &FunctionInfo, output: &mut DataChunk) -> duckdb::Result<(), Box<dyn Error>> {
        read_delta(func, output);

        Ok(())
    }

    fn parameters() -> Option<Vec<LogicalType>> {
        Some(vec![LogicalType::new(LogicalTypeId::Varchar)])
    }
}
