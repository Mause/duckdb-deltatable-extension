use build_script::cargo_rerun_if_changed;
use std::path::PathBuf;
use std::{env, path::Path};

fn main() {
    let duckdb_root = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("..")
        .join("duckdb")
        .canonicalize()
        .expect("canon");

    let header = "src/wrapper.h";

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    cargo_rerun_if_changed(&header);

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(header)
        // .enable_cxx_namespaces()
        // .generate_comments(true)
        // .derive_default(true)
        // Tell bindgen we are processing c++
        // .clang_arg("-xc++")
        // .clang_arg("-std=c++11")
        .clang_arg("-I")
        .clang_arg(duckdb_root.join("src/include").to_string_lossy())
        // .allowlist_type("duckdb::DuckDB")
        // .opaque_type("std::.*")
        .derive_debug(true)
        .derive_default(true)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
