# Variables
RUST_LIB_PATH := $(shell cd ../target/release && pwd)
GO_BUILD_FLAGS := -ldflags "-r $(RUST_LIB_PATH)"
OUTPUT_BINARY := polycrypt_ffi_go

# Default target
all: go-build go-run

# Build the Go program
go-build:
	go build $(GO_BUILD_FLAGS) -o $(OUTPUT_BINARY)

# Run the Go program
go-run: go-build
	./$(OUTPUT_BINARY)

# Clean up
go-clean:
	rm -f $(OUTPUT_BINARY)

.PHONY: all go-build go-run go-clean