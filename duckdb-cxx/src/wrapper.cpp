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

    RustCreateFunctionInfo::RustCreateFunctionInfo(ScalarFunction & scalar_function) : CreateScalarFunctionInfo(
            scalar_function
        ) {}

    std::unique_ptr<duckdb::CreateInfo> RustCreateFunctionInfo::Copy() const {
        throw std::runtime_error("can't copy me!");
//        return std::make_unique<RustCreateFunctionInfo>(this->name);
    }

    duckdb::CreateFunctionInfo *create_function_info(ScalarFunction &scalarFunction) {
        return new RustCreateFunctionInfo(scalarFunction);
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

    void setFunction(ScalarFunctionBuilder &builder, ScalarFunctionT function) {
        builder.function = function;
    }

    void setBind(ScalarFunctionBuilder &builder, BindFunctionT bind) {
        builder.bind = bind;
    }

    std::unique_ptr<Value> value_from_string(string &s) {
        return std::make_unique<Value>(s);
    }

    void vector_reference_value(Vector &autocxx_gen_this, Value &value) {
        autocxx_gen_this.Reference(value);
    }

    Value datachunk_get_value(const DataChunk &datachunk, size_t col, size_t row) {
        return datachunk.GetValue(col, row);
    }

    string value_get_string(const unique_ptr<Value> &value) {
        return value->GetValue<string>();
    }

    void ScalarFunctionBuilder::setReturnType(duckdb::LogicalType &returnType) {
        this->return_type = returnType;
    }

    std::unique_ptr<duckdb::ScalarFunction> ScalarFunctionBuilder::build() {
//        bind_scalar_function_t pFunction = (bind_scalar_function_t) bind;
        auto local_function = this->function;

        return std::make_unique<duckdb::ScalarFunction>(
                function_name, arguments, return_type, [local_function](const duckdb::DataChunk &args, const duckdb::ExpressionState &state, duckdb::Vector &result) {
                    auto res = local_function(args, state, result);

                    if (res) {
                        res->Throw();
                    }
                }
        );
    }
}
