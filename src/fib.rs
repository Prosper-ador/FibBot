use num_bigint::{BigInt, ToBigInt};
fn fibonacci(n: u64) -> BigInt {
    if n <= 1 {
        return n.to_bigint().unwrap();
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