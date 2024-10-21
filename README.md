# polycrypt-rs

polycrypt-rs is a multi-language cryptographic library wrapper designed to provide a standardized encryption and decryption solution for organizations dealing with sensitive data, such as Protected Health Information (PHI) in healthcare settings.

*A systems-level cryptographic library with cross-language FFI capabilities, designed for backend infrastructure and security-critical applications.*

## Motivation

In response to the challenges faced by a healthcare company client with a 100% PHI data layer field encryption requirement, polycrypt-rs was developed to:

1. Eliminate the need for a distributed de/encryption service, which could introduce latency and become a single point of failure.
2. Avoid the complexity and maintenance overhead of implementing separate encryption libraries for each programming language used within the organization.
3. Provide a unified, efficient, and secure cryptographic solution that can be easily integrated across various programming languages and platforms.

## Features

- AES encryption & decryption
- Field-level encryption & decryption for JSON objects
- FFI (Foreign Function Interface) bindings for Go and Python
- Logging functionality

## Requirements

- Rust 1.55 or higher
- Go 1.22 or higher (for Go bindings)
- Python 3.6 or higher (for Python bindings)

## Installation

1. Clone the repository:
   ```
   git clone https://github.com/HenryOwenz/polycrypt-rs.git
   cd polycrypt-rs
   ```

2. Build the Rust library:
   ```
   make
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

- `examples/go_example.go`: Demonstrates usage of the library in Go
- `examples/python_example.py`: Demonstrates usage of the library in Python

To run the examples:

1. For Go: `make go-run`
2. For Python: `make py-run`

### Tests

The project includes tests for Rust, Go, and Python implementations:

- Rust tests: Located in `src` and `tests` directories
- Go tests: Located in `examples/go_example_test.go`
- Python tests: Located in `examples/python_example_test.py`

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
