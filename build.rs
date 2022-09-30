extern crate bindgen;

use std::{env, path::Path};
use std::path::PathBuf;
use build_script::{cargo_rustc_link_search, cargo_rerun_if_changed, cargo_mapping, cargo_rustc_link_lib_mapping, cargo_rustc_link_lib, cargo_rustc_flags};
use build_script::cargo_rustc_link_lib::Kind;

fn main() {
    let duckdb_root = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("../duckdb").canonicalize().expect("canon");
    println!("{:?}", duckdb_root);

    // Tell cargo to look for shared libraries in the specified directory
    cargo_rustc_link_search(duckdb_root.join("build/debug/src"));

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    // cargo_rustc_link_lib_mapping(Kind::Static, "duckdb_static");
    cargo_rustc_link_lib_mapping(Kind::Static, "duckdb_static");

    // println!("cargo:rustc-link-arg=-undefined");
    // println!("cargo:rustc-link-arg=dynamic_lookup");

    // cargo_mapping("rustc-link-arg", "-undefined");
    // cargo_mapping("rustc-link-arg", "dynamic_lookup");
    let name = "libtest_extension";

    rustc_link_arg("-Wl,-undefined=dynamic_lookup");
    rustc_link_arg("-undefined=dynamic_lookup");
    // rustc_link_arg(&"-Wl,-dead_strip");
    rustc_link_arg("-Wl,--gc-sections");
    rustc_link_arg("-Wl,--gc-sections");
    rustc_link_arg("-dead_strip_dylibs");
    rustc_link_arg("-Wl,-why_live");
    rustc_link_arg("_ZN10duckdb_re23RE23Arg12parse_stringEPKcmPv");
    // cargo_rustc_flags("-Wl,-fdata-sections");
    rustc_link_arg("-ffunction-sections");
    // rustc_link_arg(&format!("-exported_symbol,_{}_init", name));
    // rustc_link_arg(&format!("-exported_symbol,_{}_version", name));
    // rustc_link_arg(&format!("-exported_symbol,_{}_replacement_open_pre,", name));
    // rustc_link_arg(&format!("-exported_symbol,_{}_replacement_open_post", name));
    // target_link_libraries(${TARGET_NAME} duckdb_static ${DUCKDB_EXTRA_LINK_FLAGS} -Wl,-dead_strip ${WHITELIST})

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    cargo_rerun_if_changed("wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")

        // .enable_cxx_namespaces()

        // .generate_comments(true)
        // .derive_default(true)
        // Tell bindgen we are processing c++
        // .clang_arg("-xc++")
        // .clang_arg("-std=c++11")
        .clang_arg("-I").clang_arg(duckdb_root.join("src/include").to_string_lossy())

        // .allowlist_type("duckdb::DuckDB")
        // .opaque_type("std::.*")
        .opaque_type("duckdb_result")
        .opaque_type("duckdb_value")

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

fn rustc_link_arg(value: impl Into<String>) {
    cargo_mapping("rustc-link-arg", value.into());
}
