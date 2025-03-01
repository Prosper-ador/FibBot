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
    let pr_number: u64 = env::var("GITHUB_PR_NUMBER")
        .expect("GITHUB_PR_NUMBER not set ")
        .parse::<u64>()
        .expect("Invalid GITHUB_PR_NUMBER");

    println!("the pull_request number is: {}",pr_number);

    println!("Enable Fibonacci: {}", enable_fib);
    println!("Max Threshold: {}", max_threshold);

    let text = "The numbers are 0.12+4, 46, and 78.";
    let numbers = extract_numbers(text);
    println!("{:?}", numbers); 

    for i in 0..numbers.len() {
                
        let b = numbers.get(i);
        println!("number: {}", b.unwrap());
    
        let a = fibonacci(*b.unwrap());
        println!("fibonacci({}) = {}", b.unwrap(), a);
    }

    match get_request_calc_and_comment().await {
        Ok(result) => println!("{}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
}
mod test;
mod fib;
use test::get_num::extract_numbers;
use crate::fib::fibonacci;
