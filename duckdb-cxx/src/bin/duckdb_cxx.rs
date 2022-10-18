use duckdb_cxx::{get_version, load_extension};

fn main() {
    println!("version: {:?}", get_version());

    load_extension();
}
