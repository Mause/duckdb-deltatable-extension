use deltalake::open_table;
use duckdb::ffi::{duckdb_decimal, duckdb_malloc, duckdb_vector_size};
use duckdb::vtab::{
    BindInfo, DataChunk, FlatVector, Free, FunctionInfo, InitInfo, Inserter, LogicalType,
    LogicalTypeId, VTab,
};
use parquet::data_type::{AsBytes, Decimal};
use std::error::Error;
use std::ffi::{CStr, CString};
use std::fs::File;
use std::mem::size_of;
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
    done: bool, // TODO: support more than *vector size* rows
}
impl Free for MyInitDataStruct {}

/// # Safety
///
/// .
fn read_delta(info: &FunctionInfo, output: &mut DataChunk) {
    let bind_data = info.get_bind_data::<MyBindDataStruct>();
    let mut init_data = info.get_init_data::<MyInitDataStruct>();

    let filename = unsafe { CStr::from_ptr((*bind_data).filename) };

    let table_result = RUNTIME.block_on(open_table(filename.to_str().unwrap()));

    if let Err(err) = table_result {
        info.set_error(&err.to_string());
        return;
    }

    let table = table_result.unwrap();

    let root_dir = Path::new(filename.to_str().unwrap());
    let mut row_idx: usize = 0;
    for pq_filename in table.get_files_iter() {
        unsafe {
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
                let mut flat_vec = output.flat_vector(col_idx);
                populate_column(value, &mut flat_vec, output, row_idx, col_idx);
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
        (*init_data).done = true;
    }
    output.set_len(row_idx);
}

fn populate_column(
    value: &Field,
    flat_vec: &mut FlatVector,
    _output: &DataChunk,
    row_idx: usize,
    _col_idx: usize,
) {
    match value {
        Field::Int(v) => {
            assign(flat_vec, row_idx, *v);
        }
        Field::Bool(v) => {
            assign(flat_vec, row_idx, *v);
        }
        Field::Long(v) => {
            assign(flat_vec, row_idx, *v);
        }
        Field::Date(v) => {
            assign(flat_vec, row_idx, *v);
        }
        Field::Float(v) => {
            assign(flat_vec, row_idx, *v);
        }
        Field::Byte(v) => {
            assign(flat_vec, row_idx, *v);
        }
        Field::Short(v) => {
            assign(flat_vec, row_idx, *v);
        }
        Field::UByte(v) => {
            assign(flat_vec, row_idx, *v);
        }
        Field::UShort(v) => {
            assign(flat_vec, row_idx, *v);
        }
        Field::UInt(v) => {
            assign(flat_vec, row_idx, *v);
        }
        Field::ULong(v) => {
            assign(flat_vec, row_idx, *v);
        }
        Field::Double(v) => {
            assign(flat_vec, row_idx, *v);
        }
        Field::Decimal(v) => match v {
            Decimal::Int64 {
                value,
                scale,
                precision,
            } => {
                assign(
                    flat_vec,
                    row_idx,
                    create_decimal(
                        value[0] as i64,
                        value[1] as u64,
                        (*precision).try_into().expect("precision"),
                        (*scale).try_into().expect("scale"),
                    ),
                );
            }
            _ => todo!("decimal"),
        },
        Field::TimestampMillis(v) => {
            assign(flat_vec, row_idx, *v);
        }
        Field::TimestampMicros(v) => {
            assign(flat_vec, row_idx, *v);
        }
        Field::Bytes(v) => {
            set_bytes(flat_vec, row_idx, v.as_bytes());
        }
        Field::Str(v) => {
            set_bytes(flat_vec, row_idx, v.as_bytes());
        }
        // TODO: support more types
        _ => todo!("unsupported type: {}", value),
    }
}

fn create_decimal(upper: i64, lower: u64, scale: u8, width: u8) -> *mut duckdb_decimal {
    let dec = malloc_c::<duckdb_decimal>();
    unsafe {
        (*dec).value.upper = upper;
        (*dec).value.lower = lower;
        (*dec).scale = scale;
        (*dec).width = width;
    }
    dec
}

fn malloc_c<T>() -> *mut T {
    unsafe { duckdb_malloc(size_of::<T>()).cast() }
}

fn set_bytes(result_vector: &mut FlatVector, row_idx: usize, bytes: &[u8]) {
    let cs = CString::new(bytes).unwrap();

    assert_eq!(result_vector.logical_type().id(), LogicalTypeId::Varchar);

    result_vector.insert(row_idx, cs);
}

fn assign<T>(flat_vec: &mut FlatVector, row_idx: usize, v: T) {
    flat_vec.as_mut_slice::<T>()[row_idx] = v;
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
    for field in schema.get_fields() {
        let typ = map_type(field.get_type());
        bind_info.add_result_column(field.get_name(), typ);
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

    fn init(_init: &InitInfo, _data: *mut Self::InitData) -> duckdb::Result<(), Box<dyn Error>> {
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
