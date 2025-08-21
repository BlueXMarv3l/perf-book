use std::time::Instant;

fn slow_operation() -> u64 {
    // Simulate work by calculating sum of squares
    let mut sum = 0;
    for i in 1..=1_000_000 {
        sum += i * i;
    }
    sum
}

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

fn benchmark_function<F>(name: &str, f: F, n: u64) 
where 
    F: Fn(u64) -> u64,
{
    let start = Instant::now();
    let result = f(n);
    let duration = start.elapsed();
    
    println!("{}: {} (took {:?})", name, result, duration);
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
    println!("=== Getting Started Examples ===\n");
    
    // Exercise 1: Basic Timing
    println!("Exercise 1: Basic Performance Timing");
    let start = Instant::now();
    let result = slow_operation();
    let duration = start.elapsed();
    println!("Result: {}", result);
    println!("Time taken: {:?}\n", duration);
    
    // Exercise 2: Comparing Algorithms
    println!("Exercise 2: Comparing Algorithms");
    let n = 1_000_000;
    benchmark_function("Loop approach", sum_with_loop, n);
    benchmark_function("Formula approach", sum_with_formula, n);
    println!();
    
    // Exercise 3: Memory Allocation
    println!("Exercise 3: Memory Allocation Patterns");
    let size = 1_000_000;
    benchmark_vec_creation("Vec::new + push", create_vec_with_push, size);
    benchmark_vec_creation("Vec::with_capacity", create_vec_with_capacity, size);
    benchmark_vec_creation("Iterator collect", create_vec_collect, size);
    println!();
    
    println!("Tip: Run with --release for optimized performance!");
}