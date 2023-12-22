/*
 * because we link twice (once to the rust library, and once to the duckdb library) we need a bridge to export the rust symbols
 * this is that bridge
 */

#include "wrapper.h"

const char* libhello_ext_version(void);
void deltatable_init_rust(void* db);

DUCKDB_EXTENSION_API const char* deltatable_version() {
    return libhello_ext_version();
}

DUCKDB_EXTENSION_API void deltatable_init(void* db) {
    deltatable_init_rust(db);
}
