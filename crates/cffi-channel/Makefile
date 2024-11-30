# Detect OS
UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S),Darwin)
  LIBNAME = libcallback.dylib
  LIBPREFIX = lib
else
  LIBNAME = libcallback.so
  LIBPREFIX = lib
endif

# Output paths
BUILD_DIR = build
OUTPUT_DIR = libs

build:
	cmake -S csrc -B $(BUILD_DIR)
	cmake --build $(BUILD_DIR) --config Release

clean:
	rm -rf $(BUILD_DIR)

copy:
	mkdir -p $(OUTPUT_DIR)
	cp $(BUILD_DIR)/$(LIBNAME) $(OUTPUT_DIR)/$(LIBNAME)

ci.build: clean build copy

.PHONY: build clean copy ci.build