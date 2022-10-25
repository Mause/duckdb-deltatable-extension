#include <iostream>
#include "wrapper.hpp"

namespace duckdb {
    std::shared_ptr <duckdb::DuckDB> new_duckdb() {
        return std::make_shared<DuckDB>(":memory:");
    }

    const char *duckdb_source_id() {
        return DuckDB::SourceID();
    }

    std::shared_ptr <DatabaseInstance> get_instance(const shared_ptr<DuckDB>& duck) {
        return duck->instance;
    }

    void set_name(CreateFunctionInfo& cfi, const std::string& name) {
        cfi.name = name;
    }

    shared_ptr<Connection> new_connection(const shared_ptr<DuckDB>& duckdb) {
        return std::make_shared<Connection>(*duckdb);
    }

    void begin_transaction(const shared_ptr<Connection>& connection) {
        connection->BeginTransaction();
    }

    void commit(const shared_ptr <Connection> &connection) {
        connection->Commit();
    }

    Catalog& get_catalog(shared_ptr<DatabaseInstance>& database_instance) {
        return database_instance->GetCatalog();
    }

    duckdb::ClientContext& get_context(std::shared_ptr<duckdb::Connection>& connection) {
        return *connection->context;
    }
}

namespace ext_framework {
    void FunctionActual(duckdb::DataChunk &, duckdb::ExpressionState &, duckdb::Vector &) {
    }

    RustCreateFunctionInfo::RustCreateFunctionInfo() : CreateScalarFunctionInfo(
            duckdb::ScalarFunction("main", {duckdb::LogicalTypeId::VARCHAR}, duckdb::LogicalTypeId::VARCHAR, FunctionActual)
        ) {}

    std::unique_ptr<duckdb::CreateInfo> RustCreateFunctionInfo::Copy() const {
        return std::make_unique<RustCreateFunctionInfo>();
    }

    duckdb::CreateFunctionInfo* create() {
        return new RustCreateFunctionInfo();
    }
    void drop_create_function_info(duckdb::CreateFunctionInfo * ptr) {
        delete ptr;
    }
}
