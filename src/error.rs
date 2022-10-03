#[macro_export]
macro_rules! check {
    ($x:expr) => {
        assert_eq!(
            $x,
            $crate::duckly::duckdb_state_DuckDBSuccess,
            "failed call: {}",
            stringify!($x)
        )
    };
}

#[macro_export]
macro_rules! as_string {
    ($x:expr) => {
        std::ffi::CString::new($x)
            .expect("c string")
            .as_ptr()
            .cast::<c_char>()
    };
}
