# Pull the official Rust image and use as the builder
FROM rust:latest AS builder

# Set the working directory in the container
WORKDIR /fibo

# Copy the Cargo manifest files to cache dependencies
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

# Build the Rust project in release mode with static linking
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl

# Use the scratch image for the final stage
FROM scratch

# Set the working directory in the container
WORKDIR /fibo

# Copy the built binary from the builder stage
COPY --from=builder /fibo/target/x86_64-unknown-linux-musl/release/fibbot /fibbot

# Set the command to run the binary(executable) on container startup
CMD [ "./fibbot" ]
