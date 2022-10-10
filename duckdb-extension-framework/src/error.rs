/// Asserts that the given expression returns DuckDBSuccess, else panics and prints the expression
#[macro_export]
macro_rules! check {
    ($x:expr) => {{
        if ($x != $crate::duckly::duckdb_state_DuckDBSuccess) {
            Err(format!("failed call: {}", stringify!($x)))?;
        }
    }};
}

/// Returns a `*const c_char` pointer to the given string
#[macro_export]
macro_rules! as_string {
    ($x:expr) => {
        std::ffi::CString::new($x)
            .expect("c string")
            .as_ptr()
            .cast::<c_char>()
    };
}
