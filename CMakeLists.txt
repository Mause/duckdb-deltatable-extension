# cmake-lint: disable=C0103

cmake_minimum_required(VERSION 3.22)
cmake_policy(VERSION 3.22)
set(TARGET_NAME deltatable)
project(${TARGET_NAME} VERSION 0.0.1)
include_directories(src/include)

if(ANDROID)
    set(Rust_CARGO_TARGET "aarch64-linux-android")
endif()

include(FetchContent)

FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG v0.4
)
FetchContent_MakeAvailable(Corrosion)

corrosion_import_crate(MANIFEST_PATH ${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml)

set(EXTENSION_NAME ${TARGET_NAME}_extension)
set(LOADABLE_EXTENSION_NAME ${TARGET_NAME}_loadable_extension)

include_directories(src/include)

set(EXTENSION_SOURCES src/deltatable_extension.cpp)

# build_static_extension(${TARGET_NAME} ${EXTENSION_SOURCES})
build_loadable_extension(${TARGET_NAME} " " ${EXTENSION_SOURCES})

set_target_properties(${LOADABLE_EXTENSION_NAME} PROPERTIES LINKER_LANGUAGE CXX)
target_link_libraries(
    ${LOADABLE_EXTENSION_NAME}
    "${CMAKE_CURRENT_BINARY_DIR}/lib${TARGET_NAME}.a"
)
# target_link_libraries(
#     ${EXTENSION_NAME}
#     "${CMAKE_CURRENT_BINARY_DIR}/lib${TARGET_NAME}.a"
# )

install(
    TARGETS ${LOADABLE_EXTENSION_NAME}
    EXPORT "${DUCKDB_EXPORT_SET}"
    LIBRARY DESTINATION "${INSTALL_LIB_DIR}"
    ARCHIVE DESTINATION "${INSTALL_LIB_DIR}"
)
