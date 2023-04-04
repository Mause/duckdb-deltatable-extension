#!/usr/bin/env fish
set -e LD_LIBRARY_PATH
set -e LD_PRELOAD
clear
cxxbridge src/defs.rs
cargo test --no-run
set -x LD_LIBRARY_PATH /home/me/duckdb-deltatable-extension/build/debug/src/
set -x LD_PRELOAD /usr/lib/gcc/x86_64-linux-gnu/11/libasan.so
cargo test