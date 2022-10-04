#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(improper_ctypes)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

extern "C" {
    #[link_name = "\u{1}_ZN6duckdb6DuckDB14LibraryVersionEv"]
    pub fn DuckDB_LibraryVersion() -> *const ::std::os::raw::c_char;
}
