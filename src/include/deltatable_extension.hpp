#pragma once

#include "duckdb.hpp"

extern "C" {
DUCKDB_EXTENSION_API void deltatable_init(duckdb::DatabaseInstance &db);

DUCKDB_EXTENSION_API const char *deltatable_version(void);
}

namespace duckdb {

class DeltatableExtension : public Extension {
public:
  void Load(DuckDB &db) override;
  std::string Name() override;
};

} // namespace duckdb
