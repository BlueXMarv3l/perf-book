# Getting Started for Entry-Level Users

*This chapter provides a foundation for Rust beginners who want to eventually 
understand performance optimization. The rest of the book assumes intermediate 
to advanced Rust knowledge.*

Welcome to your journey in Rust performance optimization! Before diving into 
the advanced techniques covered in this book, it's important to build a solid 
foundation in Rust fundamentals and basic performance concepts.

## Prerequisites

This chapter assumes you have:
- Basic programming experience in any language
- Rust installed on your system ([rustup.rs](https://rustup.rs/))
- Familiarity with the command line

## Study Project: Performance Basics

We'll build a simple project that demonstrates core Rust concepts and 
introduces basic performance measurement. This will prepare you for the 
advanced topics in the rest of the book.

### Project Setup

Create a new Rust project:
```bash
cargo new rust-perf-basics
cd rust-perf-basics
```

*Note: A complete working example of all exercises can be found in the 
`examples/rust-perf-basics` directory of this book's repository.*

### Exercise 1: Basic Timing

Let's start with measuring how long operations take. Add this to your 
`src/main.rs`:

```rust
use std::time::Instant;

fn slow_operation() -> u64 {
    // Simulate work by calculating sum of squares
    let mut sum = 0;
    for i in 1..=1_000_000 {
        sum += i * i;
    }
    sum
}

fn main() {
    println!("=== Basic Performance Timing ===\n");
    
    // Measure the time it takes to run our operation
    let start = Instant::now();
    let result = slow_operation();
    let duration = start.elapsed();
    
    println!("Result: {}", result);
    println!("Time taken: {:?}", duration);
}
```

Run this with:
```bash
cargo run
```

**Key Concepts Learned:**
- Using `std::time::Instant` for basic timing
- Understanding that performance measurement requires actual work
- Seeing how Rust handles large numbers

### Exercise 2: Comparing Algorithms

Now let's compare different approaches to the same problem:

```rust
use std::time::Instant;

fn sum_with_loop(n: u64) -> u64 {
    let mut sum = 0;
    for i in 1..=n {
        sum += i * i;
    }
    sum
}

fn sum_with_formula(n: u64) -> u64 {
    // Mathematical formula: sum of squares = n(n+1)(2n+1)/6
    n * (n + 1) * (2 * n + 1) / 6
}

fn benchmark_function<F>(name: &str, f: F, n: u64) 
where 
    F: Fn(u64) -> u64,
{
    let start = Instant::now();
    let result = f(n);
    let duration = start.elapsed();
    
    println!("{}: {} (took {:?})", name, result, duration);
}

fn main() {
    println!("=== Comparing Algorithms ===\n");
    
    let n = 1_000_000;
    
    benchmark_function("Loop approach", sum_with_loop, n);
    benchmark_function("Formula approach", sum_with_formula, n);
}
```

**Key Concepts Learned:**
- Algorithms can have dramatically different performance characteristics
- Sometimes mathematical insight can replace computation
- Function parameters and generics in Rust

### Exercise 3: Memory Allocation Basics

Let's explore how different data structures affect performance:

```rust
use std::time::Instant;

fn create_vec_with_push(size: usize) -> Vec<u32> {
    let mut vec = Vec::new();
    for i in 0..size {
        vec.push(i as u32);
    }
    vec
}

fn create_vec_with_capacity(size: usize) -> Vec<u32> {
    let mut vec = Vec::with_capacity(size);
    for i in 0..size {
        vec.push(i as u32);
    }
    vec
}

fn create_vec_collect(size: usize) -> Vec<u32> {
    (0..size).map(|i| i as u32).collect()
}

fn benchmark_vec_creation<F>(name: &str, f: F, size: usize)
where
    F: Fn(usize) -> Vec<u32>,
{
    let start = Instant::now();
    let vec = f(size);
    let duration = start.elapsed();
    
    println!("{}: created {} elements (took {:?})", 
             name, vec.len(), duration);
}

fn main() {
    println!("=== Memory Allocation Patterns ===\n");
    
    let size = 1_000_000;
    
    benchmark_vec_creation("Vec::new + push", create_vec_with_push, size);
    benchmark_vec_creation("Vec::with_capacity", create_vec_with_capacity, size);
    benchmark_vec_creation("Iterator collect", create_vec_collect, size);
}
```

**Key Concepts Learned:**
- Memory allocation can be a significant performance factor
- Pre-allocating memory with `with_capacity` can improve performance
- Iterator patterns can be both readable and efficient

### Exercise 4: Release vs Debug Builds

Run your programs with different optimization levels:

```bash
# Debug build (default)
cargo run

# Release build (optimized)
cargo run --release
```

You should see significant performance differences!

**Key Concepts Learned:**
- Compiler optimizations make a huge difference
- Always measure performance with release builds
- Debug builds are for development, release builds for production

## Next Steps

After completing these exercises, you'll have a foundation for understanding:

1. **[Build Configuration]** - How compiler settings affect performance
2. **[Benchmarking]** - More sophisticated performance measurement
3. **[Heap Allocations]** - Advanced memory management techniques
4. **[Iterators]** - Efficient data processing patterns

[Build Configuration]: build-configuration.md
[Benchmarking]: benchmarking.md
[Heap Allocations]: heap-allocations.md
[Iterators]: iterators.md

## Further Reading

Before diving into the advanced chapters, consider strengthening your Rust 
foundation with:

- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings Exercises](https://github.com/rust-lang/rustlings)

Once you're comfortable with these basics and the exercises above, you'll be 
ready to tackle the performance optimization techniques in the rest of this book!