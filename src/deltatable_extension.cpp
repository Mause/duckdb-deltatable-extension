#define DUCKDB_EXTENSION_MAIN

#include "deltatable_extension.hpp"
#include "duckdb.hpp"

extern "C" {
/*
 * because we link twice (once to the rust library, and once to the duckdb
 * library) we need a bridge to export the rust symbols this is that bridge
 */
void deltatable_init_rust(void *db);

// this name is to work around a bug in duckdb-rs
const char *libhello_ext_version(void);

DUCKDB_EXTENSION_API void deltatable_init(duckdb::DatabaseInstance &db) {
  deltatable_init_rust((void *)&db);
}

DUCKDB_EXTENSION_API const char *deltatable_version() {
  return libhello_ext_version();
}
};

#ifndef DUCKDB_EXTENSION_MAIN
#error DUCKDB_EXTENSION_MAIN not defined
#endif

void duckdb::DeltatableExtension::Load(DuckDB &db) {
  DuckDB *ptr = &db;
  deltatable_init_rust((void *)ptr);
}

std::string duckdb::DeltatableExtension::Name() { return "deltatable"; }
