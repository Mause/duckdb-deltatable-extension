use build_script::{cargo_rerun_if_changed, cargo_rustc_link_lib, cargo_rustc_link_search};
use std::env;
use std::path::Path;

fn main() -> miette::Result<()> {
    let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let base = Path::new(&cargo_manifest_dir);
    let main_file = "src/defs.rs";
    let duckdb = base.join("../duckdb/src/include");
    let src = base.join("src");

    let mut b = autocxx_build::Builder::new(main_file, &[&duckdb, &src]).build()?;
    // This assumes all your C++ bindings are in main.rs

    let wrapper = "src/wrapper.cpp";
    b.include(&duckdb)
        .include(&src)
        .files(vec![wrapper])
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-redundant-move")
        .flag_if_supported("-std=c++14")
        .compile("autocxx-demo"); // arbitrary library name, pick anything
    cargo_rerun_if_changed(main_file);
    cargo_rerun_if_changed(wrapper);
    cargo_rerun_if_changed("src/wrapper.hpp");

    cargo_rustc_link_lib("duckdb");
    cargo_rustc_link_lib("ubsan");
    cargo_rustc_link_lib("asan");
    cargo_rustc_link_search("/home/me/duckdb-deltatable-extension/build/debug/src");
    // Add instructions to link to any C++ libraries you need.
    Ok(())
}
