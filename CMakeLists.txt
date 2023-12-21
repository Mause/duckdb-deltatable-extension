# cmake-lint: disable=C0103

cmake_minimum_required(VERSION 3.22)
cmake_policy(VERSION 3.22)
set(TARGET_NAME deltatable)
project(${TARGET_NAME} VERSION 0.0.1)

if(ANDROID)
    set(Rust_CARGO_TARGET "aarch64-linux-android")
endif()

include(FetchContent)

fetchcontent_declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG
        v0.2.1 # Optionally specify a commit hash, version tag or branch here
)
fetchcontent_makeavailable(Corrosion)

corrosion_import_crate(MANIFEST_PATH ${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml)

set(ALL_SOURCES src/wrapper.h src/wrapper.c)

build_loadable_extension(${TARGET_NAME} ${ALL_SOURCES})

set(LIB_NAME ${TARGET_NAME}_loadable_extension)

set_target_properties(${LIB_NAME} PROPERTIES LINKER_LANGUAGE CXX)
target_link_libraries(
    ${LIB_NAME}
    "${CMAKE_CURRENT_BINARY_DIR}/lib${TARGET_NAME}.a"
)
