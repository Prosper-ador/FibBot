FROM rust:slim

# Set the working directory in the container
WORKDIR /fibo

# Copy the Cargo manifest files to cache dependencies
COPY . .

RUN cargo build --release

# Build the Rust project in release mode with static linking
CMD [ "./target/release/fibbot" ]