# Variables
CARGO := cargo
MAKE := make
TARGET_DIR := target
RELEASE_DIR := $(TARGET_DIR)/release
DEBUG_DIR := $(TARGET_DIR)/debug
BINARY_NAME := polycrypt_rs
RUST_LIB_PATH := $(shell pwd)/$(RELEASE_DIR)
GO_BUILD_FLAGS := -ldflags "-r $(RUST_LIB_PATH)"
GO_OUTPUT_BINARY := polycrypt_ffi_go
EXAMPLES_DIR := examples
GO_EXAMPLES_DIR := $(EXAMPLES_DIR)/go
PYTHON_EXAMPLES_DIR := $(EXAMPLES_DIR)/python
DB_SETUP_DIR := db/setup
GO_ENTRY_POINT := main.go
PYTHON_ENTRY_POINT := main.py
PYTHON_TEST_ENTRY_POINT := test.py

# Determine OS-specific variables
ifeq ($(OS),Windows_NT)
    LIB_EXT := dll
else
    UNAME_S := $(shell uname -s)
    ifeq ($(UNAME_S),Linux)
        LIB_EXT := so
    endif
    ifeq ($(UNAME_S),Darwin)
        LIB_EXT := dylib
        # For cross-compilation from Mac
        CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER := x86_64-linux-musl-gcc
        export CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER
    endif
endif

LIB_NAME := libpolycrypt_rs.$(LIB_EXT)
LINUX_LIB_NAME := libpolycrypt_rs.so

# Colors
CYAN := \033[36m
GREEN := \033[32m
YELLOW := \033[33m
RED := \033[31m
RESET_COLOR := \033[0m

# Aesthetic
DASH_LINE := \033[90m---------------------------------------------------------------\033[0m

# Default target
# all: build

# Build for release and copy the library
build: 
	@echo "$(DASH_LINE)"
	@echo "$(CYAN)Building polycrypt-rs (release)...$(RESET_COLOR)"
	@echo "$(DASH_LINE)"
	@$(CARGO) build --release
	@echo "$(DASH_LINE)"
	@echo "$(GREEN)Build completed successfully.$(RESET_COLOR)"
	@echo "$(DASH_LINE)"
	@echo "$(CYAN)Copying $(LIB_NAME) to Go and Python packages...$(RESET_COLOR)"
	@cp $(RELEASE_DIR)/$(LIB_NAME) $(GO_EXAMPLES_DIR)/polycrypt/
	@cp $(RELEASE_DIR)/$(LIB_NAME) $(PYTHON_EXAMPLES_DIR)/polycrypt/
	@cp $(RELEASE_DIR)/$(LIB_NAME) $(DB_SETUP_DIR)/polycrypt/
	@echo "$(GREEN)Library copied successfully.$(RESET_COLOR)"
	@echo "$(DASH_LINE)"

# Add cross-compilation target
build-linux:
	@echo "$(DASH_LINE)"
	@echo "$(CYAN)Cross-compiling for Linux x86_64...$(RESET_COLOR)"
	@echo "$(DASH_LINE)"
	@rustup target add x86_64-unknown-linux-musl
	@RUSTFLAGS="-C target-feature=-crt-static" $(CARGO) build --release --target x86_64-unknown-linux-musl
	@echo "$(GREEN)Linux build completed successfully.$(RESET_COLOR)"
	@echo "$(DASH_LINE)"
	@echo "$(CYAN)Copying $(LINUX_LIB_NAME) to staging directory...$(RESET_COLOR)"
	@mkdir -p $(RELEASE_DIR)/linux
	@cp $(TARGET_DIR)/x86_64-unknown-linux-musl/release/$(LINUX_LIB_NAME) $(RELEASE_DIR)/linux/
	@echo "$(GREEN)Linux library copied successfully.$(RESET_COLOR)"
	@echo "$(DASH_LINE)"

# Add a combined build target for both native and Linux
build-all: build build-linux

# Build for debug
debug:
	@echo "$(DASH_LINE)"
	@echo "$(CYAN)Building polycrypt-rs (debug)...$(RESET_COLOR)"
	@echo "$(DASH_LINE)"
	@$(CARGO) build
	@echo "$(DASH_LINE)"
	@echo "$(GREEN)Debug build completed successfully.$(RESET_COLOR)"
	@echo "$(DASH_LINE)"

# Run the program (release mode)
run: build
	@echo "$(DASH_LINE)"
	@echo "$(CYAN)Running polycrypt-rs (release)...$(RESET_COLOR)"
	@echo "$(DASH_LINE)"
	@$(RELEASE_DIR)/$(BINARY_NAME)
	@echo "$(DASH_LINE)"

# Run the program (debug mode)
run-debug: debug
	@echo "$(DASH_LINE)"
	@echo "$(CYAN)Running polycrypt-rs (debug)...$(RESET_COLOR)"
	@echo "$(DASH_LINE)"
	@$(DEBUG_DIR)/$(BINARY_NAME)
	@echo "$(DASH_LINE)"

# Run tests
test:
	@echo "$(DASH_LINE)"
	@echo "$(CYAN)Running Rust tests for polycrypt-rs...$(RESET_COLOR)"
	@echo "$(DASH_LINE)"
	@$(CARGO) test
	@echo "$(DASH_LINE)"
	@echo "$(GREEN)Rust tests completed.$(RESET_COLOR)"
	@echo "$(DASH_LINE)"

# Run benchmarks
bench:
	@echo "$(DASH_LINE)"
	@echo "$(CYAN)Running benchmarks for polycrypt-rs...$(RESET_COLOR)"
	@echo "$(DASH_LINE)"
	@$(CARGO) bench --bench db_benchmarks
	@echo "$(DASH_LINE)"
	@echo "$(GREEN)Benchmarks completed.$(RESET_COLOR)"
	@echo "$(DASH_LINE)"

bench-heavy:
	@echo "$(DASH_LINE)"
	@echo "$(CYAN)Running heavy benchmarks...$(RESET_COLOR)"
	@echo "$(DASH_LINE)"
	@ENABLE_HEAVY_BENCHMARKS=1 $(CARGO) bench
	@echo "$(DASH_LINE)"
	@echo "$(GREEN)Heavy benchmarks completed.$(RESET_COLOR)"
	@echo "$(DASH_LINE)"

# Check the project for errors
check:
	@echo "$(DASH_LINE)"
	@echo "$(CYAN)Checking polycrypt-rs for errors...$(RESET_COLOR)"
	@echo "$(DASH_LINE)"
	@$(CARGO) check
	@echo "$(DASH_LINE)"
	@echo "$(GREEN)Check completed.$(RESET_COLOR)"
	@echo "$(DASH_LINE)"

# Format the code
fmt:
	@echo "$(DASH_LINE)"
	@echo "$(CYAN)Formatting polycrypt-rs code...$(RESET_COLOR)"
	@echo "$(DASH_LINE)"
	@$(CARGO) fmt
	@echo "$(DASH_LINE)"
	@echo "$(GREEN)Formatting completed.$(RESET_COLOR)"
	@echo "$(DASH_LINE)"

# Lint the code
lint:
	@echo "$(DASH_LINE)"
	@echo "$(CYAN)Linting polycrypt-rs code...$(RESET_COLOR)"
	@echo "$(DASH_LINE)"
	@$(CARGO) clippy
	@echo "$(DASH_LINE)"
	@echo "$(GREEN)Linting completed.$(RESET_COLOR)"
	@echo "$(DASH_LINE)"

# Clean the project
clean:
	@echo "$(DASH_LINE)"
	@echo "$(CYAN)Cleaning polycrypt-rs project...$(RESET_COLOR)"
	@echo "$(DASH_LINE)"
	@$(CARGO) clean
	@rm -f $(GO_EXAMPLES_DIR)/polycrypt/libpolycrypt_rs.*
	@rm -f $(GO_EXAMPLES_DIR)/polycrypt_ffi_go
	@rm -f $(PYTHON_EXAMPLES_DIR)/polycrypt/libpolycrypt_rs.*
	@rm -rf $(RELEASE_DIR)/linux
	@echo "$(DASH_LINE)"
	@echo "$(GREEN)Cleaning completed.$(RESET_COLOR)"
	@echo "$(DASH_LINE)"

# Generate documentation
doc:
	@echo "$(DASH_LINE)"
	@echo "$(CYAN)Generating documentation for polycrypt-rs...$(RESET_COLOR)"
	@echo "$(DASH_LINE)"
	@$(CARGO) doc --no-deps
	@echo "$(DASH_LINE)"
	@echo "$(GREEN)Documentation generated.$(RESET_COLOR)"
	@echo "$(DASH_LINE)"

# FFI bindings tests
py-run: build
	@echo "$(DASH_LINE)"
	@echo "$(CYAN)Running Python example...$(RESET_COLOR)"
	@echo "$(DASH_LINE)"
	@RUST_LOG=debug python3 $(PYTHON_EXAMPLES_DIR)/$(PYTHON_ENTRY_POINT)
	@echo "$(DASH_LINE)"
	@echo "$(GREEN)Python example completed.$(RESET_COLOR)"
	@echo "$(DASH_LINE)"

# Build the Go example
go-build: build
	@echo "$(DASH_LINE)"
	@echo "$(CYAN)Building Go example...$(RESET_COLOR)"
	@echo "$(DASH_LINE)"
	@cd $(GO_EXAMPLES_DIR) && go build $(GO_BUILD_FLAGS) -o $(GO_OUTPUT_BINARY) $(GO_ENTRY_POINT)
	@echo "$(DASH_LINE)"
	@echo "$(GREEN)Go example built successfully.$(RESET_COLOR)"
	@echo "$(DASH_LINE)"

# Run the Go example
go-run: go-build
	@echo "$(DASH_LINE)"
	@echo "$(CYAN)Running Go example...$(RESET_COLOR)"
	@echo "$(DASH_LINE)"
	@cd $(GO_EXAMPLES_DIR) && ./$(GO_OUTPUT_BINARY)
	@echo "$(DASH_LINE)"
	@echo "$(GREEN)Go example completed.$(RESET_COLOR)"
	@echo "$(DASH_LINE)"

# Run Go tests (including example tests)
go-test: build
	@echo "$(DASH_LINE)"
	@echo "$(CYAN)Running Go tests for polycrypt-rs...$(RESET_COLOR)"
	@echo "$(DASH_LINE)"
	@cd $(GO_EXAMPLES_DIR) && LD_LIBRARY_PATH=$(RUST_LIB_PATH) go test -v $(GO_BUILD_FLAGS)
	@echo "$(DASH_LINE)"
	@echo "$(GREEN)Go tests completed.$(RESET_COLOR)"
	@echo "$(DASH_LINE)"

# Run Python tests (including example tests)
py-test: build
	@echo "$(DASH_LINE)"
	@echo "$(CYAN)Running Python tests for polycrypt-rs...$(RESET_COLOR)"
	@echo "$(DASH_LINE)"
	@cd $(PYTHON_EXAMPLES_DIR) && python3 -m unittest $(PYTHON_TEST_ENTRY_POINT)
	@echo "$(DASH_LINE)"
	@echo "$(GREEN)Python tests completed.$(RESET_COLOR)"
	@echo "$(DASH_LINE)"

# Run all tests (Rust, Go, and Python)
test-all: 
	@echo "$(DASH_LINE)"
	@echo "$(YELLOW)Running all tests for polycrypt-rs...$(RESET_COLOR)"
	@echo "$(DASH_LINE)"
	@echo "$(CYAN)Running Rust tests...$(RESET_COLOR)"
	@echo "$(DASH_LINE)"
	@$(CARGO) test
	@echo "$(DASH_LINE)"
	@echo "$(GREEN)Rust tests completed.$(RESET_COLOR)"
	@echo "$(DASH_LINE)"
	@echo "$(CYAN)Running Go tests...$(RESET_COLOR)"
	@echo "$(DASH_LINE)"
	@cd $(GO_EXAMPLES_DIR) && LD_LIBRARY_PATH=$(RUST_LIB_PATH) go test -v $(GO_BUILD_FLAGS)
	@echo "$(DASH_LINE)"
	@echo "$(GREEN)Go tests completed.$(RESET_COLOR)"
	@echo "$(DASH_LINE)"
	@echo "$(CYAN)Running Python tests...$(RESET_COLOR)"
	@echo "$(DASH_LINE)"
	@cd $(PYTHON_EXAMPLES_DIR) && python3 -m unittest $(PYTHON_TEST_ENTRY_POINT)
	@echo "$(DASH_LINE)"
	@echo "$(GREEN)Python tests completed.$(RESET_COLOR)"
	@echo "$(DASH_LINE)"
	@echo "$(YELLOW)All tests for polycrypt-rs completed.$(RESET_COLOR)"
	@echo "$(DASH_LINE)"

.PHONY: all build debug run run-debug test bench check fmt lint clean doc install uninstall py-run go-build go-run go-test py-test test-all
