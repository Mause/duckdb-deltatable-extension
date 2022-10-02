#[macro_export]
macro_rules! check {
    ($x:expr) => {
        assert_eq!($x, $crate::duckly::duckdb_state_DuckDBSuccess)
    };
}
