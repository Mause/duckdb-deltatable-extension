.PHONY: all clean format debug release duckdb_debug duckdb_release pull update

all: release

MKFILE_PATH := $(abspath $(lastword $(MAKEFILE_LIST)))
PROJ_DIR := $(dir $(MKFILE_PATH))

OSX_BUILD_UNIVERSAL_FLAG=
ifeq (${OSX_BUILD_UNIVERSAL}, 1)
	OSX_BUILD_UNIVERSAL_FLAG=-DOSX_BUILD_UNIVERSAL=1
endif

#### Enable Ninja as generator
ifeq ($(GEN),ninja)
	GENERATOR=-G "Ninja" -DFORCE_COLORED_OUTPUT=1
endif

EXT_NAME=deltatable

#### Configuration for this extension
EXTENSION_NAME=DELTATABLE
EXTENSION_FLAGS=\
-DDUCKDB_EXTENSION_NAMES="deltatable" \
-DDUCKDB_EXTENSION_${EXTENSION_NAME}_PATH="$(PROJ_DIR)" \
-DDUCKDB_EXTENSION_${EXTENSION_NAME}_LOAD_TESTS=0 \
-DDUCKDB_EXTENSION_${EXTENSION_NAME}_INCLUDE_PATH="$(PROJ_DIR)src/include" \
-DDUCKDB_EXTENSION_${EXTENSION_NAME}_TEST_PATH="$(PROJ_DIR)test/sql"

BUILD_FLAGS=-DEXTENSION_STATIC_BUILD=1 -DBUILD_SHELL=0 $(EXTENSION_FLAGS) ${EXTRA_EXTENSIONS_FLAG} $(OSX_BUILD_FLAG) $(TOOLCHAIN_FLAGS)
CLIENT_FLAGS:=

CLIENT_FLAGS=-DDUCKDB_EXTENSION_${EXTENSION_NAME}_SHOULD_LINK=0


pull:
	git submodule init
	# git submodule update --recursive --remote

clean:
	rm -rf build
	rm -rf test/simple_table*
	cargo clean

debug:
	mkdir -p  build/debug && \
	cmake $(GENERATOR) $(BUILD_FLAGS) $(CLIENT_FLAGS) -DCMAKE_BUILD_TYPE=Debug -S ./duckdb/ -B build/debug && \
	cmake --build build/debug --config Debug

release:
	mkdir -p build/release && \
	cmake $(GENERATOR) $(BUILD_FLAGS)  $(CLIENT_FLAGS)  -DCMAKE_BUILD_TYPE=Release -S ./duckdb/ -B build/release && \
	cmake --build build/release --config Release

test_release: release simple_tables
	./build/release/test/unittest --test-dir . "[sql]"

POPULATE=build/debug/extension/deltatable/populate

test/simple_table:
	$(POPULATE) test/simple_table

test/simple_table_2:
	$(POPULATE) test/simple_table_2 --with-list

simple_tables: test/simple_table test/simple_table_2

test: debug simple_tables
	./build/debug/test/unittest --test-dir . "[sql]"

update:
	git submodule update --remote --merge
