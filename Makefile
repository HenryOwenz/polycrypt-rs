# Variables
CARGO := cargo
TARGET_DIR := target
RELEASE_DIR := $(TARGET_DIR)/release
DEBUG_DIR := $(TARGET_DIR)/debug
BINARY_NAME := polycrypt_rs

# Default target
all: build

# Build for release
build: 
	$(CARGO) build --release

# Build for debug
debug:
	$(CARGO) build

# Run the program (release mode)
run: build
	$(RELEASE_DIR)/$(BINARY_NAME)

# Run the program (debug mode)
run-debug: debug
	$(DEBUG_DIR)/$(BINARY_NAME)

# Run tests
test:
	$(CARGO) test

# Run benchmarks
bench:
	$(CARGO) bench

# Check the project for errors
check:
	$(CARGO) check

# Format the code
fmt:
	$(CARGO) fmt

# Lint the code
lint:
	$(CARGO) clippy

# Clean the project
clean:
	$(CARGO) clean

# Generate documentation
doc:
	$(CARGO) doc --no-deps

# Install the binary
install: build
	$(CARGO) install --path .

# Uninstall the binary
uninstall:
	$(CARGO) uninstall $(BINARY_NAME)

# FFI bindings tests
py-run:
	RUST_LOG=debug python3 examples/python_example.py

.PHONY: all build debug run run-debug test bench check fmt lint clean doc install uninstall
