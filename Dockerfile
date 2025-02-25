 FROM rust:latest 

WORKDIR /action

COPY . .

RUN cargo build --release

ENTRYPOINT ["/action/target/release/fibbot"]