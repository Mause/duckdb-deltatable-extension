//
// Created by me on 15/10/22.
//

#include "wrapper.hpp"
#include "duckdb/parser/parsed_data/create_function_info.hpp"

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

//    void CreateFunction(DatabaseInstance &di, shared_ptr <CreateFunctionInfo> cfi, ClientContext &context) {
//        auto & catalog = Catalog::GetCatalog(di);
//        Connection conn(di);
//        catalog.CreateFunction(
//                context,
//                &cfi
//                );
//    }

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

    shared_ptr <CreateFunctionInfo> create_function_info() {
//        return new CreateFunctionInfo();
        return NULL;
    }

    Catalog& get_catalog(shared_ptr<DatabaseInstance>& database_instance) {
        return database_instance->GetCatalog();
    }

    duckdb::ClientContext& get_context(std::shared_ptr<duckdb::Connection>& connection) {
        return *connection->context;
    }
}