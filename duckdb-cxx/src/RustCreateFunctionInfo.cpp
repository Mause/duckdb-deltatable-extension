//
// Created by me on 18/10/22.
//

#include <iostream>
#include "RustCreateFunctionInfo.h"


namespace ext_framework {
    RustCreateFunctionInfo::RustCreateFunctionInfo() : CreateFunctionInfo(duckdb::CatalogType::SCALAR_FUNCTION_ENTRY, "main") {
        std::cout << "in constructor" << std::endl;
        this->name = "hello";
        this->schema = "main";
    }

    void RustCreateFunctionInfo::SetFunction(const std::string &name) {
        this->name = name;
    }

    std::shared_ptr<RustCreateFunctionInfo> create() {
        return {};
    }
}
