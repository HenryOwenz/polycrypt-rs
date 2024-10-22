# Variables
CARGO := cargo
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
GO_ENTRY_POINT := main.go
PYTHON_ENTRY_POINT := main.py
PYTHON_TEST_ENTRY_POINT := test.py

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

# Build for release
build: 
	@echo "$(DASH_LINE)"
	@echo "$(CYAN)Building polycrypt-rs (release)...$(RESET_COLOR)"
	@echo "$(DASH_LINE)"
	@$(CARGO) build --release
	@echo "$(DASH_LINE)"
	@echo "$(GREEN)Build completed successfully.$(RESET_COLOR)"
	@echo "$(DASH_LINE)"

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
	@$(CARGO) bench
	@echo "$(DASH_LINE)"
	@echo "$(GREEN)Benchmarks completed.$(RESET_COLOR)"
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
	@rm -f $(EXAMPLES_DIR)/$(GO_OUTPUT_BINARY)
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

# Install the binary
install: build
	@echo "$(DASH_LINE)"
	@echo "$(CYAN)Installing polycrypt-rs...$(RESET_COLOR)"
	@echo "$(DASH_LINE)"
	@$(CARGO) install --path .
	@echo "$(DASH_LINE)"
	@echo "$(GREEN)Installation completed.$(RESET_COLOR)"
	@echo "$(DASH_LINE)"

# Uninstall the binary
uninstall:
	@echo "$(DASH_LINE)"
	@echo "$(CYAN)Uninstalling polycrypt-rs...$(RESET_COLOR)"
	@echo "$(DASH_LINE)"
	@$(CARGO) uninstall $(BINARY_NAME)
	@echo "$(DASH_LINE)"
	@echo "$(GREEN)Uninstallation completed.$(RESET_COLOR)"
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
