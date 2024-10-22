# Packaging and Distribution Guide for polycrypt-rs

This guide outlines the strategy for packaging and distributing polycrypt-rs, including its Rust core and language wrappers for Go and Python.

## Current Approach

For the initial release, we are adopting the following approach:

1. **Single Repository Structure**: 
   - Maintain all components (Rust core, Go wrapper, Python wrapper) in a single repository.
   - This simplifies version management and ensures consistency across all parts of the library.

2. **Language-Specific Directories**:
   - Rust core: `src/`
   - Go wrapper: `examples/go/polycrypt/`
   - Python wrapper: `examples/python/polycrypt/`

3. **Distribution Strategy**:
   - Rust:
     - Publish the core library on crates.io.
     - Include pre-built binaries for common platforms in GitHub releases.
   - Go:
     - Maintain as a Go module within the repository.
     - Provide clear instructions for users to include it in their Go projects.
   - Python:
     - Create a simple `setup.py` for local installation.
     - Do not publish on PyPI yet.

4. **Version Management**:
   - Use semantic versioning for all components.
   - Clearly document compatibility between core and wrapper versions.

5. **Documentation**:
   - Maintain comprehensive documentation in the `docs/` directory.
   - Provide clear installation and usage instructions for each language in the README.

## Future Considerations

As the project grows, consider the following improvements:

1. **Separate Language Packages**:
   - Create separate packages for Go and Python wrappers.
   - This allows for language-specific versioning and easier integration with language-specific package managers.

2. **Automated Builds and Releases**:
   - Implement CI/CD pipelines for automated testing and releases.
   - Automate the process of building for different platforms.

3. **Package Registry Publishing**:
   - Publish Go package on pkg.go.dev.
   - Publish Python package on PyPI.

4. **Versioning Strategy**:
   - Develop a clear strategy for managing versions across the core library and language wrappers.

5. **Performance Benchmarks**:
   - Include performance comparisons with other encryption solutions.

6. **Extended Language Support**:
   - Consider adding support for additional programming languages based on user demand.

## Installation Instructions

### Rust
