.PHONY: all clean format debug release duckdb_debug duckdb_release pull update

all: release

OSX_BUILD_UNIVERSAL_FLAG=
ifeq (${OSX_BUILD_UNIVERSAL}, 1)
	OSX_BUILD_UNIVERSAL_FLAG=-DOSX_BUILD_UNIVERSAL=1
endif

ifeq ($(GEN),ninja)
	GENERATOR=-G "Ninja"
	FORCE_COLOR=-DFORCE_COLORED_OUTPUT=1
endif

BUILD_FLAGS=-DEXTENSION_STATIC_BUILD=1 ${OSX_BUILD_UNIVERSAL_FLAG}

pull:
	git submodule init
	git submodule update --recursive --remote

clean:
	rm -rf build

debug: pull
	mkdir -p build/debug && \
	cd build/debug && \
	cmake $(GENERATOR) $(FORCE_COLOR) -DCMAKE_BUILD_TYPE=Debug ${BUILD_FLAGS} ../../duckdb/CMakeLists.txt -DEXTERNAL_EXTENSION_DIRECTORIES=../../test_extension -B. && \
	cmake --build . --config Debug

release: pull
	mkdir -p build/release && \
	cd build/release && \
	cmake $(GENERATOR) $(FORCE_COLOR) -DCMAKE_BUILD_TYPE=RelWithDebInfo ${BUILD_FLAGS} ../../duckdb/CMakeLists.txt -DEXTERNAL_EXTENSION_DIRECTORIES=../../test_extension -B. && \
	cmake --build . --config Release

test_release:
	./build/release/test/unittest --test-dir . "[sql]"

test:
	./duckdb/build/debug/test/unittest --test-dir . "[sql]"

update:
	git submodule update --remote --merge
