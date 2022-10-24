#pragma once
#include <functional>
#define DUCKDB_BUILD_LOADABLE_EXTENSION
#include "duckdb.hpp"
#include "duckdb/parser/parsed_data/create_function_info.hpp"
#include "duckdb/parser/parsed_data/create_scalar_function_info.hpp"

namespace duckdb {
    std::shared_ptr<duckdb::DuckDB> new_duckdb();
    const char* duckdb_source_id();
    std::shared_ptr<DatabaseInstance> get_instance(const shared_ptr<DuckDB>& duck);
    void set_name(CreateFunctionInfo& cfi, const std::string& name);

    shared_ptr<Connection> new_connection(const shared_ptr<DuckDB>& duckdb);
    void begin_transaction(const shared_ptr<Connection>& connection);
    void commit(const shared_ptr<Connection>& connection);
    shared_ptr<CreateFunctionInfo> create_function_info();

    Catalog& get_catalog(shared_ptr<DatabaseInstance>& database_instance);

    duckdb::ClientContext& get_context(std::shared_ptr<duckdb::Connection>&);
}

namespace ext_framework {
    class RustCreateFunctionInfo : public duckdb::CreateScalarFunctionInfo {
    public:
        DUCKDB_API explicit RustCreateFunctionInfo();

        [[nodiscard]] std::unique_ptr<duckdb::CreateInfo> Copy() const override;
    };

    duckdb::CreateFunctionInfo* create();
}
