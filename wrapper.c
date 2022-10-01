#include "wrapper.h"

const char* libtest_extension_version_v2();
void libtest_extension_init_v2(void* db);

DUCKDB_EXTENSION_API const char* libtest_extension_version() {
    printf("hello world version\n");
    return libtest_extension_version_v2();
}

DUCKDB_EXTENSION_API void libtest_extension_init(void* db) {
    printf("hello world init\n");
    libtest_extension_init_v2(db);
}
