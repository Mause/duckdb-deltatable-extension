/*
 * because we link twice (once to the rust library, and once to the duckdb library) we need a bridge to export the rust symbols
 * this is that bridge
 */

#include "wrapper.h"

const char* deltatable_version_rust(void);
void deltatable_init_rust(void* db);

DUCKDB_EXTENSION_API const char* deltatable_version() {
    return deltatable_version_rust();
}

DUCKDB_EXTENSION_API void deltatable_init(void* db) {
    deltatable_init_rust(db);
}
