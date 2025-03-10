/*use std::env;
use octocrab::Octocrab;
use crate::test::get_num::extract_numbers;


async fn request() {
    let github_token = env::var("GITHUB_TOKEN").expect("Missing GITHUB_TOKEN");
    let repo = env::var("GITHUB_REPOSITORY").expect("GITHUB_REPOSITORY not set");
    let owner = repo.split('/').next().unwrap();
    let pr_number = match env::var("GITHUB_PR_NUMBER") {
        Ok(value) => match value.parse::<u64>() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Invalid PR number: '{}'. Expected a positive integer.", value);
                std::process::exit(1); // Gracefully exit with an error code
            }
        },
        Err(_) => {
            eprintln!("Missing GITHUB_PR_NUMBER. Please set it before running.");
            std::process::exit(1);
        }
    };
    
    let octocrab = Octocrab::builder().personal_token(github_token).build().unwrap();

    let comments_url = format!("/repos/{owner}/{repo}/pulls/{pr_number}");
    let pr_response: serde_json::Value = octocrab.get(&comments_url).await.unwrap();

    let pr_content = pr_response["body"].as_str().unwrap_or("");
    let numbers = extract_numbers(pr_content);

    println!("Extracted numbers: {:?}", numbers);
}*/

use std::env;
use octocrab::Octocrab;
use crate::{fib::fibonacci, test::get_num::extract_numbers};

pub async fn get_request_calc_and_comment() -> Result<String, Box<dyn std::error::Error>> {
    let github_token = env::var("GITHUB_TOKEN").expect("Missing GITHUB_TOKEN");
    let repo = env::var("GITHUB_REPOSITORY").expect("GITHUB_REPOSITORY not set");
    let args: Vec<String> = env::args().collect();

    // Expecting the first argument (after the binary name) to be the PR number.
    if args.len() < 2 || args[1].trim().is_empty() {
        eprintln!("Missing PR number argument.");
        std::process::exit(1);
    }

    let pr_number: u64 = args[1].trim().parse().expect("Invalid PR number");
    println!("PR Number from argument: {}", pr_number);

    // Optionally, read from environment if needed.
    let env_pr = env::var("GITHUB_PR_NUMBER").unwrap_or_default();
    println!("PR Number from env: {}", env_pr);
    
    let (owner, repo_name) = repo
        .split_once('/')
        .expect("Invalid GITHUB_REPOSITORY format");

    let octocrab = Octocrab::builder().personal_token(github_token).build()?;

    let pr = octocrab.pulls(owner, repo_name).get(pr_number).await?;
    let pr_body = pr.body.unwrap_or_else(|| "No description".to_string());

    let numbers = extract_numbers(&pr_body);
    let _result = if numbers.is_empty() {
        "No numbers found in the PR description.".to_string()
    } else {
        format!("### Extracted Numbers \n{:?}", numbers)
    };
    let mut results = vec![];
    for num in &numbers {
        let fib_value = fibonacci(*num);
        println!("Number: {} → Fibonacci({}) = {}", num, num, fib_value);
        results.push(format!("- Fibonacci({}) = {}", num, fib_value));
    }

    //let response = format!("### Extracted Numbers & Fibonacci \n{}", results.join("\n"));
    //println!("### Extracted Numbers & Fibonacci \n");
    
    //Ok(results.join("\n"))
    let comment_body = format!(
        "### Fibonacci PR Scanner Bot \n\
         Extracted Numbers: `{:?}`\n\n\
         {}\n\n\
         Generated automatically by FibBot_",
        numbers, results.join("\n")
    );

    // **Post the comment to GitHub**
    octocrab
        .issues(owner, repo_name)
        .create_comment(pr_number, comment_body.clone())
        .await?;

    println!("Comment posted successfully!");
    Ok(comment_body)
}
