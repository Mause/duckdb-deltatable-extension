//
// Created by me on 18/10/22.
//

#pragma once
#define DUCKDB_BUILD_LOADABLE_EXTENSION
#include "duckdb.hpp"
#include "duckdb/parser/parsed_data/create_function_info.hpp"

namespace ext_framework {
    class RustCreateFunctionInfo : public duckdb::CreateFunctionInfo {
    public:
        DUCKDB_API explicit RustCreateFunctionInfo();

        DUCKDB_API void SetFunction(std::string const& name);
    };

    std::shared_ptr<RustCreateFunctionInfo> create();
}
