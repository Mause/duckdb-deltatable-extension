/*
 * because we link twice (once to the rust library, and once to the duckdb library) we need a bridge to export the rust symbols
 * this is that bridge
 */

#include "wrapper.h"

const char* libtest_extension_version_rust(void);
void libtest_extension_init_rust(void* db);

DUCKDB_EXTENSION_API const char* libtest_extension_version() {
    return libtest_extension_version_rust();
}

DUCKDB_EXTENSION_API void libtest_extension_init(void* db) {
    libtest_extension_init_rust(db);
}
