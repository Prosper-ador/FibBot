use std::env;
use crate::test::get_pr::get_request_calc_and_comment;

#[tokio::main]
async fn main() {
    println!("Good Morning");
    // Read input parameters
    let enable_fib = env::var("INPUT_ENABLE_FIB").unwrap_or_else(|_| "true".to_string()) == "true";
    let max_threshold: u64 = env::var("INPUT_MAX_THRESHOLD")
        .unwrap_or_else(|_| "1000".to_string())
        .parse()
        .unwrap_or(1000);
    let pr_number: u64 = env::var("GITHUB_PR_NUMBER").expect("msg").parse().unwrap();
    println!("Enable Fibonacci: {}", enable_fib);
    println!("Max Threshold: {}", max_threshold);
    println!("PR Number: {}", pr_number);

    match get_request_calc_and_comment(env::var("GITHUB_PR_NUMBER").unwrap().parse().unwrap()).await {
        Ok(result) => println!("{}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
}
mod test;
mod fib;

