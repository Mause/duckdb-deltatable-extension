all: target/debug/libtest_extension.so
	../duckdb/build/debug/duckdb -unsigned -c "load 'target/debug/libtest_extension.so'"

target/debug/libtest_extension.so:
	cargo build

clean:
	cargo clean

other:
	rustc -C link-arg=-undefined -C link-arg=dynamic_lookup src/lib.rs
