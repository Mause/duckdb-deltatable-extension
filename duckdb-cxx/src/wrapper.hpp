#pragma once
#include <functional>
#define DUCKDB_BUILD_LOADABLE_EXTENSION
#include "duckdb.hpp"
#include "duckdb/parser/parsed_data/create_function_info.hpp"
#include "duckdb/parser/parsed_data/create_scalar_function_info.hpp"
#include "duckdb/common/types.hpp"
#include <cxx.h>

namespace duckdb {
    std::shared_ptr<DuckDB> new_duckdb();

    const char* duckdb_source_id();
    void set_name(CreateFunctionInfo& cfi, const std::string& name);

    shared_ptr<Connection> new_connection(DatabaseInstance& duckdb);
    void begin_transaction(const shared_ptr<Connection>& connection);
    void commit(const shared_ptr<Connection>& connection);

    Catalog& get_catalog(DatabaseInstance& database_instance);

    duckdb::ClientContext& get_context(std::shared_ptr<duckdb::Connection>&);
    DatabaseInstance* get_instance(const std::shared_ptr<DuckDB>& duck);

    void duckdb_LogicalType_new1_autocxx_wrapper(duckdb::LogicalType* autocxx_gen_this, duckdb::LogicalTypeId arg1);

    class RustCreateFunctionInfo : public duckdb::CreateScalarFunctionInfo {
    public:
        DUCKDB_API explicit RustCreateFunctionInfo(ScalarFunction &scalar_function);

        [[nodiscard]] std::unique_ptr<duckdb::CreateInfo> Copy() const override;
    };

    class ScalarFunctionBuilder {
    public:
        explicit ScalarFunctionBuilder(const std::string &function_name, duckdb::LogicalType &returnType);
        std::unique_ptr<duckdb::ScalarFunction> build();

        void setArguments(const std::vector<duckdb::LogicalType> &arguments);

        void setReturnType(duckdb::LogicalType &returnType);

        void addArgument(duckdb::LogicalType& arg);

        rust::Fn<void(const duckdb::DataChunk &, const duckdb::ExpressionState &, duckdb::Vector &)> function;
        rust::Fn<FunctionData(ClientContext &context, ScalarFunction &bound_function,
                              vector<unique_ptr<Expression>> &arguments)> bind;
    private:
        const std::string &function_name;
        std::vector<duckdb::LogicalType> arguments;
        duckdb::LogicalType& return_type;
    };

    duckdb::LogicalType* create_logical_type(duckdb::LogicalTypeId typ);

    duckdb::CreateFunctionInfo* create_function_info(ScalarFunction& scalarFunction);
    void drop_create_function_info(duckdb::CreateFunctionInfo* ptr);

    void vector_print(const duckdb::Vector& autocxx_gen_this);
    void vector_reference_value(duckdb::Vector& autocxx_gen_this, Value& value);

    void setBind(
        duckdb::ScalarFunctionBuilder& builder,
        rust::cxxbridge1::Fn<void(const duckdb::DataChunk&, const duckdb::ExpressionState&, duckdb::Vector&)> bind);
    void setFunction(
        duckdb::ScalarFunctionBuilder& builder,
        rust::cxxbridge1::Fn<void(const duckdb::DataChunk&, const duckdb::ExpressionState&, duckdb::Vector&)> function
    );

    std::unique_ptr<Value> value_from_string(string& s);
}
