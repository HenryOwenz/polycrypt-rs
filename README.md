# polycrypt-rs

polycrypt-rs is a multi-language cryptographic library wrapper designed to provide a standardized encryption and decryption solution for organizations dealing with sensitive data, such as Protected Health Information (PHI) in healthcare settings.

## Motivation

In response to the challenges faced by a healthcare company client with a 100% PHI data layer field encryption requirement, polycrypt-rs was developed to:

1. Eliminate the need for a distributed de/encryption service, which could introduce latency and become a single point of failure.
2. Avoid the complexity and maintenance overhead of implementing separate encryption libraries for each programming language used within the organization.
3. Provide a unified, efficient, and secure cryptographic solution that can be easily integrated across various programming languages and platforms.

## Features

- AES encryption and decryption
- Field-level encryption for JSON objects
- FFI bindings for Go and Python
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
   cargo build --release
   ```

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