use crate::duckly::{
    duckdb_create_data_chunk, duckdb_data_chunk, duckdb_data_chunk_get_column_count,
    duckdb_data_chunk_get_size, duckdb_data_chunk_get_vector, duckdb_data_chunk_reset,
    duckdb_data_chunk_set_size, duckdb_logical_type, idx_t,
};
use crate::{LogicalType, Vector};

pub struct DataChunk(duckdb_data_chunk);

impl DataChunk {
    /// Creates an empty DataChunk with the specified set of types.
    ///
    /// # Arguments
    /// - `types`: An array of types of the data chunk.
    pub fn new(types: &[LogicalType]) -> Self {
        let types: Vec<duckdb_logical_type> = types.iter().map(|x| x.typ).collect();
        let mut types = types.into_boxed_slice();

        Self(unsafe {
            duckdb_create_data_chunk(types.as_mut_ptr(), types.len().try_into().unwrap())
        })
    }
    pub fn get_vector(&self, p0: u64) -> Vector {
        Vector::from(unsafe { duckdb_data_chunk_get_vector(self.0, p0) })
    }
    pub fn set_size(&self, size: u64) {
        unsafe { duckdb_data_chunk_set_size(self.0, size) };
    }
    pub fn chunk_reset(&self) {
        unsafe { duckdb_data_chunk_reset(self.0) }
    }
    pub fn get_column_count(&self) -> idx_t {
        unsafe { duckdb_data_chunk_get_column_count(self.0) }
    }
    pub fn get_size(&self) -> idx_t {
        unsafe { duckdb_data_chunk_get_size(self.0) }
    }
}

impl From<duckdb_data_chunk> for DataChunk {
    fn from(ptr: duckdb_data_chunk) -> Self {
        Self(ptr)
    }
}

// impl Drop for DataChunk {
//     fn drop(&mut self) {
//         unsafe {
//             duckdb_destroy_data_chunk(&mut self.0);
//         }
//     }
// }
