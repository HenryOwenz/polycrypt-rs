# Benchmarking in polycrypt-rs

This document outlines the benchmarking setup for polycrypt-rs, explaining how we measure the performance of our cryptographic operations.

## Benchmark Setup

We use the `criterion` crate for benchmarking, which provides a statistically sound way to measure and compare the performance of our code. The benchmarks are defined in `benches/encryption_benchmarks.rs`.

### Benchmark Functions

1. `bench_encrypt`: Measures the performance of basic encryption.
2. `bench_decrypt`: Measures the performance of basic decryption.
3. `bench_encrypt_fields`: Measures the performance of field-level encryption in a JSON object.
4. `bench_decrypt_fields`: Measures the performance of field-level decryption in a JSON object.
5. `bench_encrypt_fields_in_batch`: Measures the performance of batch field-level encryption for multiple JSON objects.
6. `bench_decrypt_fields_in_batch`: Measures the performance of batch field-level decryption for multiple JSON objects.

### Key Points

- We use a fixed key (`[0u8; 32]`) for all benchmarks to ensure consistency.
- The `black_box` function is used to prevent the compiler from optimizing away the benchmarked code.
- For field-level operations, we use a sample JSON object with both sensitive and non-sensitive fields.
- Batch operations are tested with an array of two JSON objects.

## Running Benchmarks

To run the benchmarks, use the following command:
