use crate::duckly::{
    duckdb_create_data_chunk, duckdb_data_chunk, duckdb_data_chunk_get_column_count,
    duckdb_data_chunk_get_size, duckdb_data_chunk_get_vector, duckdb_data_chunk_reset,
    duckdb_data_chunk_set_size, duckdb_destroy_data_chunk, duckdb_logical_type, idx_t,
};
use crate::{LogicalType, Vector};

pub struct DataChunk {
    ptr: duckdb_data_chunk,
    owned: bool,
}

impl DataChunk {
    /// Creates an empty DataChunk with the specified set of types.
    ///
    /// # Arguments
    /// - `types`: An array of types of the data chunk.
    pub fn new(types: &[LogicalType]) -> Self {
        let types: Vec<duckdb_logical_type> = types.iter().map(|x| x.typ).collect();
        let mut types = types.into_boxed_slice();

        let ptr = unsafe {
            duckdb_create_data_chunk(types.as_mut_ptr(), types.len().try_into().unwrap())
        };

        Self { ptr, owned: true }
    }
    pub fn get_vector(&self, p0: u64) -> Vector {
        Vector::from(unsafe { duckdb_data_chunk_get_vector(self.ptr, p0) })
    }
    pub fn set_size(&self, size: u64) {
        unsafe { duckdb_data_chunk_set_size(self.ptr, size) };
    }
    pub fn chunk_reset(&self) {
        unsafe { duckdb_data_chunk_reset(self.ptr) }
    }
    pub fn get_column_count(&self) -> idx_t {
        unsafe { duckdb_data_chunk_get_column_count(self.ptr) }
    }
    pub fn get_size(&self) -> idx_t {
        unsafe { duckdb_data_chunk_get_size(self.ptr) }
    }
}

impl From<duckdb_data_chunk> for DataChunk {
    fn from(ptr: duckdb_data_chunk) -> Self {
        Self { ptr, owned: false }
    }
}

impl Drop for DataChunk {
    fn drop(&mut self) {
        if self.owned {
            unsafe { duckdb_destroy_data_chunk(&mut self.ptr) };
        }
    }
}
