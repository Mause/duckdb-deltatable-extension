#include <memory>
#include "wrapper.hpp"
#include "duckdb.hpp"

void FunctionActual(duckdb::DataChunk &args, duckdb::ExpressionState & state, duckdb::Vector & result) {
    duckdb::Value actual("hello");
    result.Reference(actual);
}

namespace duckdb {
    std::shared_ptr<DuckDB> new_duckdb() {
        return std::make_shared<DuckDB>(":memory:");
    }

    const char *duckdb_source_id() {
        return DuckDB::SourceID();
    }

    DatabaseInstance* get_instance(const shared_ptr<DuckDB>& duck) {
        return &*duck->instance;
    }

    void set_name(CreateFunctionInfo& cfi, const std::string& name) {
        cfi.name = name;
    }

    shared_ptr<Connection> new_connection(DatabaseInstance& duckdb) {
        return std::make_shared<Connection>(duckdb);
    }

    void begin_transaction(const shared_ptr<Connection>& connection) {
        connection->BeginTransaction();
    }

    void commit(const shared_ptr <Connection> &connection) {
        connection->Commit();
    }

    Catalog& get_catalog(DatabaseInstance& database_instance) {
        return database_instance.GetCatalog();
    }

    duckdb::ClientContext& get_context(std::shared_ptr<duckdb::Connection>& connection) {
        return *connection->context;
    }
    void duckdb_LogicalType_new1_autocxx_wrapper(duckdb::LogicalType* autocxx_gen_this, duckdb::LogicalTypeId arg1) {
        new (autocxx_gen_this) duckdb::LogicalType(arg1);
    }

    RustCreateFunctionInfo::RustCreateFunctionInfo(std::string function_name) : CreateScalarFunctionInfo(
            duckdb::ScalarFunction(std::move(function_name), {duckdb::LogicalTypeId::VARCHAR}, duckdb::LogicalTypeId::VARCHAR, FunctionActual)
        ) {}

    std::unique_ptr<duckdb::CreateInfo> RustCreateFunctionInfo::Copy() const {
        return std::make_unique<RustCreateFunctionInfo>(this->name);
    }

    duckdb::CreateFunctionInfo *create_function_info(std::string name) {
        return new RustCreateFunctionInfo(std::move(name));
    }
    void drop_create_function_info(duckdb::CreateFunctionInfo * ptr) {
        delete ptr;
    }

    ScalarFunctionBuilder::ScalarFunctionBuilder(const std::string &function_name, duckdb::LogicalType &returnType)
            : function_name(function_name), return_type(returnType) {
    }

    void ScalarFunctionBuilder::addArgument(duckdb::LogicalType &arg) {
        this->arguments.emplace_back(arg);
    }

    void vector_print(const Vector &autocxx_gen_this) {
        Printer::Print(autocxx_gen_this.ToString());
    }

    void setFunction(ScalarFunctionBuilder &builder,
                             rust::cxxbridge1::Fn<void(const duckdb::DataChunk &, const duckdb::ExpressionState &,
                                                       duckdb::Vector &)> function) {
        builder.function = function;
    }

    void setBind(ScalarFunctionBuilder &builder,
                         rust::cxxbridge1::Fn<void(const duckdb::DataChunk &, const duckdb::ExpressionState &,
                                                   duckdb::Vector &)> bind) {
//        builder.bind = bind;
    }

    void vector_reference_value(Vector &autocxx_gen_this, const Value &value) {
        autocxx_gen_this.Reference(value);
    }

    void ScalarFunctionBuilder::setReturnType(duckdb::LogicalType &returnType) {
        this->return_type = returnType;
    }

    void ScalarFunctionBuilder::setArguments(const std::vector<duckdb::LogicalType> &arguments) {
        this->arguments = arguments;
    }

    std::unique_ptr<duckdb::ScalarFunction> ScalarFunctionBuilder::build() {
//        void (*actual)(duckdb::DataChunk &, duckdb::ExpressionState &, duckdb::Vector &) = FunctionActual;
        return std::make_unique<duckdb::ScalarFunction>(
                function_name, arguments, return_type, function
        );
    }
}
