This is a **README.md** file for a FibBot project:

---

# FibBot

The **FibBot** bot is a GitHub Action written in Rust that extracts numbers from pull request descriptions, calculates Fibonacci numbers for each extracted number (with a maximmu threshold set to limit the number for which the fibonnaci can be calculated), and posts the results as a comment on the pull_request. We can be run both locally and as a GitHub Action.

## Features

- **Extract Numbers:** extract numerical values from the .
- **Fibonacci Calculation:** The Fibonacci numbers for each extracted number from the pull request description is computed whereby the number does not exceed a specified maximum threshold.
- **Configurable:** Enable or disable Fibonacci calculation, set a max threshold, and provide your GitHub token via inputs.
- **GitHub Integration:** Automatically post computed results as comments on pull_requests.
- **Flexible Deployment:** Run locally, with docker using dockerfile, or directly as a GitHub Action.

## Requirements

- **Rust:** Install Rust following the steps in the rust site.
- **Docker (Optional):** If you want to run in a container.
- **GitHub Personal Access Token (PAT):** With appropriate permissions for interacting with the GitHub API.

## Startup

run the following commands from your terminal

```bash
git clone https://github.com/Prosper-ador/FibBot.git
cd FibBot
```

### Building Locally

Build the project in release mode:

```bash
cargo build --release
```

This generates a binnary file at `./target/release/fibbot`.

## Running Locally

You can run the binary directly by passing the required arguments. The program expects four arguments in order: **PR_Number:** **Enable_Fibonacci Flag:** **Max_Threshold:** **GitHub Token:**

Example:

```bash
./target/release/fibbot "2" "true" "1000" "your_github_token"
```

## Docker Usage

If you prefer Docker, export the required variables then use the provided Dockerfile, do:

```bash
docker build -t fibbot .
```

```bash
docker run --rm -it fibbot "42" "true" "1000" "your_github_token"
```

## GitHub Action

FibBot is designed to run as a GitHub Action. The project includes an `action.yml` file defining inputs such as:

- **`pr_number`**: The pull request number to process.
- **`enable_fib`**: To Enable or disable Fibonacci calculation.
- **`max_threshold`**: Maximum threshold for Fibonacci calculation.
- **`github_token`**: GitHub token for API interactions.

## Contributions

Look forward to receive issues.

## Conclusion
Thank you for passing through
