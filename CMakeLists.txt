cmake_minimum_required(VERSION 3.22)
cmake_policy(VERSION 3.22)
set(TARGET_NAME libtest_extension)
project(${TARGET_NAME} VERSION 0.0.1)

set(EXTENSION_STATIC_BUILD TRUE)
enable_testing()

include(FetchContent)

FetchContent_Declare(
  Corrosion
  GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
  GIT_TAG v0.2.1 # Optionally specify a commit hash, version tag or branch here
)
FetchContent_MakeAvailable(Corrosion)

corrosion_import_crate(MANIFEST_PATH ${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml)

include(${CMAKE_CURRENT_SOURCE_DIR}/function.cmake)

add_library(mylib SHARED IMPORTED)
set_target_properties(
  mylib
  PROPERTIES IMPORTED_LOCATION
             ${CMAKE_CURRENT_SOURCE_DIR}/target/debug/libtest_extension.so)

set(ALL_SOURCES wrapper.h)

link_directories(../duckdb/build/debug/src)

build_loadable_extension(${TARGET_NAME} ${ALL_SOURCES})

set(LIB_NAME ${TARGET_NAME}_loadable_extension)

set_target_properties(${LIB_NAME} PROPERTIES LINKER_LANGUAGE CXX)
target_link_libraries(${LIB_NAME} mylib)

add_test(
  basic ../duckdb/build/debug/duckdb -unsigned -c
  "load '${CMAKE_CURRENT_SOURCE_DIR}/libtest_extension.duckdb_extension'")
