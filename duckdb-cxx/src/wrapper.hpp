#pragma once
#include <functional>
#define DUCKDB_BUILD_LOADABLE_EXTENSION
#include "duckdb.hpp"
#include "duckdb/parser/parsed_data/create_function_info.hpp"
#include "duckdb/parser/parsed_data/create_scalar_function_info.hpp"
#include "duckdb/common/types.hpp"

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

    void duckdb_LogicalType_new1_autocxx_wrapper(duckdb::LogicalType* autocxx_gen_this, duckdb::LogicalTypeId arg1);
}

namespace ext_framework {
    class RustCreateFunctionInfo : public duckdb::CreateScalarFunctionInfo {
    public:
        DUCKDB_API explicit RustCreateFunctionInfo(std::string function_name);

        [[nodiscard]] std::unique_ptr<duckdb::CreateInfo> Copy() const override;
    };

    class ScalarFunctionBuilder {
    public:
        explicit ScalarFunctionBuilder(const std::string &function_name, duckdb::LogicalType &returnType);
        std::unique_ptr<duckdb::ScalarFunction> build();

        void setArguments(const std::vector<duckdb::LogicalType> &arguments);

        void setReturnType(duckdb::LogicalType &returnType);

        void setBind(duckdb::bind_scalar_function_t bind);

//        void setFunction(const duckdb::scalar_function_t &function);

        void addArgument(duckdb::LogicalType& arg);

    private:
        const std::string &function_name;
        std::vector<duckdb::LogicalType> arguments;
        duckdb::LogicalType& return_type;
        duckdb::bind_scalar_function_t bind;
        duckdb::scalar_function_t function;
    };

    duckdb::LogicalType* create_logical_type(duckdb::LogicalTypeId typ);

    duckdb::CreateFunctionInfo* create_function_info(std::string function_name);
    void drop_create_function_info(duckdb::CreateFunctionInfo* ptr);
}
