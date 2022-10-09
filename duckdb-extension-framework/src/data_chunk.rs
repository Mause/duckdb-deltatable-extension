use crate::duckly::{duckdb_data_chunk, duckdb_data_chunk_get_vector, duckdb_data_chunk_set_size};
use crate::Vector;

pub struct DataChunk(duckdb_data_chunk);

impl DataChunk {
    pub fn get_vector(&self, p0: u64) -> Vector {
        Vector::from(unsafe { duckdb_data_chunk_get_vector(self.0, p0) })
    }
    pub fn set_size(&self, size: u64) {
        unsafe { duckdb_data_chunk_set_size(self.0, size) };
    }
}

impl From<duckdb_data_chunk> for DataChunk {
    fn from(ptr: duckdb_data_chunk) -> Self {
        Self(ptr)
    }
}
