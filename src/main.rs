use std::env;

fn main() {
    println!("Good Morning");
    // Read input parameters
    let enable_fib = env::var("INPUT_ENABLE_FIB").unwrap_or_else(|_| "true".to_string()) == "true";
    let max_threshold: u64 = env::var("INPUT_MAX_THRESHOLD")
        .unwrap_or_else(|_| "1000".to_string())
        .parse()
        .unwrap_or(1000);

    println!("Enable Fibonacci: {}", enable_fib);
    println!("Max Threshold: {}", max_threshold);
}

/*mod fib;
use regex::Regex;

use octocrab::models::IssueComment;
use octocrab::Octocrab;
use tokio;

#[tokio::main]
async fn main() {
    let github_token = env::var("GITHUB_TOKEN").expect("Missing GITHUB_TOKEN");
    let pr_number: u64 = env::var("GITHUB_PR_NUMBER")
        .expect("Missing GITHUB_PR_NUMBER")
        .parse()
        .unwrap();

    let octocrab = Octocrab::builder().personal_token(github_token).build().unwrap();

    // Fetch PR content
    let comments_url = format!("/repos/{owner}/{repo}/pulls/{pr_number}");
    let pr_response: serde_json::Value = octocrab.get(&comments_url).await.unwrap();
    let pr_content = pr_response["body"].as_str().unwrap_or("");

    // Extract numbers
    let numbers = extract_numbers(pr_content);

    // Compute Fibonacci numbers
    let enable_fib = env::var("INPUT_ENABLE_FIB").unwrap_or("true".to_string()) == "true";
    let max_threshold: u64 = env::var("INPUT_MAX_THRESHOLD").unwrap_or("1000".to_string()).parse().unwrap_or(1000);

    let mut results = Vec::new();
    for num in numbers {
        if num > max_threshold {
            continue;
        }
        let fib = fibonacci(num);
        results.push(format!("Fib({}) = {}", num, fib));
    }

    // Post comment with results
    let comment_body = format!("### Fibonacci Results 🚀\n\n{}", results.join("\n"));
    let comment_url = format!("/repos/{owner}/{repo}/issues/{pr_number}/comments");

    let _comment: IssueComment = octocrab.post(&comment_url).body(&comment_body).send().await.unwrap();

    println!("Comment posted: \n{}", comment_body);
}
    
    fn extract_numbers(text: &str) -> Vec<u64> {
    let re = Regex::new(r"\b\d+\b").unwrap();
    re.find_iter(text)
        .filter_map(|m| m.as_str().parse::<u64>().ok())
        .collect()
}*/
