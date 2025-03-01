use std::env;
use num_bigint::{BigInt, ToBigInt};
pub fn fibonacci(n: u64) -> BigInt {
    if n <= 1 {
        return n.to_bigint().unwrap();
    }
    let max_threshold: u64 = env::var("INPUT_MAX_THRESHOLD")
        .unwrap_or_else(|_| "1000".to_string())
        .parse()
        .unwrap_or(1000);
    if n > max_threshold {
        println!("Skipping number {} (exceeds max threshold of {})", n, max_threshold);
        return 0.to_bigint().unwrap();
    }
    
    let mut a  = 0.to_bigint().unwrap();
    let mut b  = 1.to_bigint().unwrap();
    let mut result  = 0.to_bigint().unwrap();

    for _ in 2..=n {
        result = a + b.clone();
        a = b;
        b = result.clone();
    }

    result.to_bigint().unwrap()
}