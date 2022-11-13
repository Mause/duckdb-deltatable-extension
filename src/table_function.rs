use deltalake::open_table;
use duckdb_extension_framework::constants::LogicalTypeId;
use duckdb_extension_framework::duckly::{
    duckdb_bind_info, duckdb_data_chunk, duckdb_free, duckdb_function_info, duckdb_init_info,
    duckdb_vector_size,
};
use duckdb_extension_framework::{
    malloc_struct, BindInfo, DataChunk, FunctionInfo, InitInfo, LogicalType, TableFunction,
};
use parquet::data_type::AsBytes;
use std::ffi::{c_void, CStr, CString};
use std::fs::File;
use std::os::raw::c_char;
use std::path::Path;
use std::slice;
use tokio::runtime::Runtime;

use crate::types::map_type;
use parquet::file::reader::SerializedFileReader;
use parquet::record::Field;

#[repr(C)]
struct MyBindDataStruct {
    filename: *mut c_char,
}

#[repr(C)]
struct MyInitDataStruct {
    done: bool, // TODO: support more than *vector size* rows
}

/// # Safety
///
/// .
#[no_mangle]
unsafe extern "C" fn read_delta(info: duckdb_function_info, output: duckdb_data_chunk) {
    let info = FunctionInfo::from(info);
    let output = DataChunk::from(output);

    let bind_data = info.get_bind_data::<MyBindDataStruct>();
    let mut init_data = info.get_init_data::<MyInitDataStruct>();

    let filename = CStr::from_ptr((*bind_data).filename);

    let table_result = RUNTIME.block_on(open_table(filename.to_str().unwrap()));

    if let Err(err) = table_result {
        info.set_error(&err.to_string());
        return;
    }

    let table = table_result.unwrap();

    let root_dir = Path::new(filename.to_str().unwrap());
    let mut row_idx: usize = 0;
    for pq_filename in table.get_files_iter() {
        if (*init_data).done {
            break;
        }
        let reader =
            SerializedFileReader::new(File::open(root_dir.join(pq_filename)).unwrap()).unwrap();

        for row in reader {
            for (col_idx, (_key, value)) in row.get_column_iter().enumerate() {
                populate_column(value, &output, row_idx, col_idx);
            }
            row_idx += 1;

            assert!(
                row_idx < duckdb_vector_size().try_into().unwrap(),
                "overflowed vector: {}",
                row_idx
            );
        }
    }
    (*init_data).done = true;
    output.set_size(row_idx as u64);
}

unsafe fn populate_column(value: &Field, output: &DataChunk, row_idx: usize, col_idx: usize) {
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

unsafe fn set_bytes(output: &DataChunk, row_idx: usize, col_idx: usize, bytes: &[u8]) {
    let cs = CString::new(bytes).unwrap();

    let result_vector = output.get_vector(col_idx as u64);

    result_vector.assign_string_element_len(row_idx as u64, cs.as_ptr(), bytes.len() as u64);
}

unsafe fn assign<T: 'static>(output: &DataChunk, row_idx: usize, col_idx: usize, v: T) {
    get_column_result_vector::<T>(output, col_idx)[row_idx] = v;
}

unsafe fn get_column_result_vector<T>(output: &DataChunk, column_index: usize) -> &'static mut [T] {
    let result_vector = output.get_vector(column_index as u64);
    let ptr = result_vector.get_data().cast::<T>();
    slice::from_raw_parts_mut(ptr, duckdb_vector_size() as usize)
}

unsafe extern "C" fn drop_my_bind_data_struct(v: *mut c_void) {
    let actual = v.cast::<MyBindDataStruct>();
    drop(CString::from_raw((*actual).filename.cast()));
    duckdb_free(v);
}

/// # Safety
///
/// .
#[no_mangle]
unsafe extern "C" fn read_delta_bind(bind_info: duckdb_bind_info) {
    let bind_info = BindInfo::from(bind_info);
    assert_eq!(bind_info.get_parameter_count(), 1);

    let param = bind_info.get_parameter(0);
    let ptr = param.get_varchar();
    let cstring = ptr.to_str().unwrap();

    let handle = RUNTIME.block_on(open_table(cstring));
    if let Err(err) = handle {
        bind_info.set_error(&err.to_string());
        return;
    }

    let table = handle.unwrap();
    let schema = table.schema().expect("no schema");
    for field in schema.get_fields() {
        let typ = LogicalType::new(map_type(field.get_type()));
        bind_info.add_result_column(field.get_name(), typ);
    }

    let my_bind_data = malloc_struct::<MyBindDataStruct>();
    (*my_bind_data).filename = CString::new(cstring).expect("c string").into_raw();

    bind_info.set_bind_data(my_bind_data.cast(), Some(drop_my_bind_data_struct));
}

/// # Safety
///
/// .
#[no_mangle]
unsafe extern "C" fn read_delta_init(info: duckdb_init_info) {
    let info = InitInfo::from(info);

    let mut my_init_data = malloc_struct::<MyInitDataStruct>();
    (*my_init_data).done = false;
    info.set_init_data(my_init_data.cast(), Some(duckdb_free));
}

pub fn build_table_function_def() -> TableFunction {
    let table_function = TableFunction::new();
    table_function.set_name("read_delta");
    let logical_type = LogicalType::new(LogicalTypeId::Varchar);
    table_function.add_parameter(&logical_type);

    table_function.set_function(Some(read_delta));
    table_function.set_init(Some(read_delta_init));
    table_function.set_bind(Some(read_delta_bind));
    table_function
}

lazy_static::lazy_static! {
    static ref RUNTIME: Runtime = tokio::runtime::Builder::new_current_thread()
            .build()
            .expect("runtime");
}
