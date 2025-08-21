# Examples

This directory contains practical examples to accompany the book chapters.

## rust-perf-basics

A complete Rust project demonstrating the exercises from the "Getting Started for Entry-Level Users" chapter. This project shows:

- Basic performance timing with `std::time::Instant`
- Algorithm comparison (O(n) loop vs O(1) formula)
- Memory allocation patterns (Vec creation methods)
- The difference between debug and release builds

### Running the examples:

```bash
cd examples/rust-perf-basics

# Run with debug build (slower)
cargo run

# Run with release build (optimized)
cargo run --release
```

The release build will show much better performance, demonstrating the importance of compiler optimizations.