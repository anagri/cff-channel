# Windows specific Makefile using PowerShell
SHELL := pwsh.exe
LIBNAME = callback.dll
BUILD_DIR = build
OUTPUT_DIR = libs

build:
	@pwsh -Command "cmake -S csrc -B $(BUILD_DIR)"
	@pwsh -Command "cmake --build $(BUILD_DIR)"

clean:
	@pwsh -Command "if (Test-Path $(BUILD_DIR)) { Remove-Item -Recurse -Force $(BUILD_DIR) }"

copy:
	@pwsh -Command "if (-not (Test-Path $(OUTPUT_DIR))) { New-Item -ItemType Directory -Force $(OUTPUT_DIR) }"
	@pwsh -Command "Copy-Item $(BUILD_DIR)/$(LIBNAME) -Destination $(OUTPUT_DIR)/$(LIBNAME) -Force"

ci.build: clean build copy

.PHONY: build clean copy ci.build
