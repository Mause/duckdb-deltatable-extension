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

    class RustFunctionData : public FunctionData {};

    typedef rust::Fn<std::unique_ptr<PreservedError>(const duckdb::DataChunk &, const duckdb::ExpressionState &, duckdb::Vector &)> ScalarFunctionT;
    typedef rust::Fn<unique_ptr<RustFunctionData>(const ClientContext &context, const ScalarFunction &bound_function, rust::Slice<const unique_ptr<Expression>> &arguments)> BindFunctionT;

    class ScalarFunctionBuilder {
    public:
        explicit ScalarFunctionBuilder(const std::string &function_name, duckdb::LogicalType &returnType);
        std::unique_ptr<duckdb::ScalarFunction> build();

        void setReturnType(duckdb::LogicalType &returnType);

        void addArgument(duckdb::LogicalType& arg);

        ScalarFunctionT function;
        BindFunctionT bind;
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

    void set_bind(duckdb::ScalarFunctionBuilder& builder, BindFunctionT bind);
    void set_function(duckdb::ScalarFunctionBuilder& builder, ScalarFunctionT function);

    Value datachunk_get_value(const duckdb::DataChunk& datachunk, size_t col, size_t row);

    std::unique_ptr<Value> value_from_string(string& s);
    string value_get_string(const unique_ptr<Value>& value);
}
