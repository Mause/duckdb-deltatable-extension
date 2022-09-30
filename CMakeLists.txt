cmake_minimum_required(VERSION 2.8.12)
set(TARGET_NAME libtest_extension)
project(${TARGET_NAME})

include(${CMAKE_CURRENT_SOURCE_DIR}/function.cmake)

add_library( mylib SHARED IMPORTED )
set_target_properties( mylib PROPERTIES IMPORTED_LOCATION ${CMAKE_CURRENT_SOURCE_DIR}/target/debug/libtest_extension.so)

set(ALL_SOURCES wrapper.h)

build_loadable_extension(${TARGET_NAME} ${ALL_SOURCES})

set_target_properties(${TARGET_NAME}_loadable_extension PROPERTIES LINKER_LANGUAGE CXX)
TARGET_LINK_LIBRARIES(${TARGET_NAME}_loadable_extension mylib)
