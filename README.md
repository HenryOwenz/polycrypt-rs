# polycrypt-rs

polycrypt-rs is a multi-language cryptographic library wrapper designed to provide a standardized encryption and decryption solution for organizations dealing with sensitive data, such as Protected Health Information (PHI) in healthcare settings.

*A systems-level cryptographic library with cross-language FFI capabilities, designed for backend infrastructure and security-critical applications.*

## Motivation

polycrypt-rs was developed in response to the complex challenges faced by a healthcare company with stringent data protection requirements:

1. Enable the creation of a secure data lake with 100% field-level encryption of PHI at rest and in transit.
2. Secure sensitive data flowing through event buses and queues without compromising system performance.
3. Provide developers the freedom to write solutions and services in their preferred languages while maintaining consistent encryption standards.
4. Eliminate the need for a distributed de/encryption service, which could introduce latency and become a single point of failure.
5. Avoid the complexity and maintenance overhead of implementing separate encryption libraries for each programming language used within the organization.

## Key Benefits

- **Language Agnostic**: Allows developers to work in their preferred languages (currently supporting Rust, Go, and Python) while maintaining consistent encryption standards.
- **Performance**: Written in Rust, ensuring high performance, low memory footprint, and the potential for future optimizations through concurrency.
- **Scalability**: Handles large volumes of data efficiently by running in the same process, avoiding network overhead.
- **Flexibility**: Supports field-level encryption in JSON objects, ideal for securing specific data elements in complex structures.
- **Compliance Ready**: Designed to meet stringent data protection requirements, such as 100% PHI encryption in healthcare settings.
- **Future-Proof**: The Rust foundation provides a pathway for future performance enhancements and feature additions.
- **Big Data Friendly**: Efficient encryption and decryption make it suitable for big data querying and processing scenarios.

## Features

- AES encryption & decryption
- Field-level encryption & decryption for JSON objects
- Batch encryption & decryption for multiple records
- FFI (Foreign Function Interface) bindings for Go and Python
- Native language wrappers for Go and Python
- Logging functionality

## Native Language Libraries

As part of our commitment to making polycrypt-rs easily accessible across different programming languages, we now maintain native language libraries for Go and Python. These libraries provide a more idiomatic interface to the underlying Rust functionality:

- **Go**: Located in `examples/go/polycrypt/polycrypt.go`
- **Python**: Located in `examples/python/polycrypt/polycrypt.py`

These native libraries wrap the FFI calls and provide a more natural API for each language, making it easier for developers to integrate polycrypt-rs into their projects without dealing with the complexities of FFI directly.

## Requirements

- Rust 1.55 or higher
- Go 1.22 or higher (for Go bindings)
- Python 3.6 or higher (for Python bindings)
- Criterion 0.6.0 or higher (for benchmarking)
- Linux build-essentials (for building on Linux)

## Cross Platform Development
### macos (arm64)
```bash
# Install the musl cross-compiler
brew install FiloSottile/musl-cross/musl-cross

# Add the musl target to rust
rustup target add x86_64-unknown-linux-musl
```

## Installation

1. Clone the repository:
   ```
   git clone https://github.com/HenryOwenz/polycrypt-rs.git
   cd polycrypt-rs
   ```

2. Build the Rust library:
   ```
   make build
   ```

## Usage

### Using the Makefile

The project includes a Makefile that simplifies common tasks. Here are some useful commands:

- `make build`: Build the project in release mode
- `make debug`: Build the project in debug mode
- `make test`: Run Rust tests
- `make go-test`: Run Go tests
- `make py-test`: Run Python tests
- `make test-all`: Run all tests (Rust, Go, and Python)
- `make go-run`: Run the Go example
- `make py-run`: Run the Python example
- `make clean`: Clean the project
- `make doc`: Generate documentation

To use these commands, simply run `make <command>` in the project root directory.

### Examples

The `examples` directory contains sample code for using polycrypt-rs with Go and Python:

- `examples/go/main.go`: Demonstrates usage of the library in Go
- `examples/python/main.py`: Demonstrates usage of the library in Python

To run the examples:

1. For Go: `make go-run`
2. For Python: `make py-run`

### Tests

The project includes tests for Rust, Go, and Python implementations:

- Rust tests: Located in `src` and `tests` directories
- Go tests: Located in `examples/go/main_test.go`
- Python tests: Located in `examples/python/main_test.py`

To run the tests:

1. For Rust: `make test`
2. For Go: `make go-test`
3. For Python: `make py-test`
4. For all tests: `make test-all`

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## Authors

- Ugochukwu Henry Onwuzurike - Initial work - [GitHub](https://github.com/HenryOwenz)

## Acknowledgments

- The Rust community for providing excellent cryptographic libraries
