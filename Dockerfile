FROM rust:latest AS builder

COPY . .
 
WORKDIR /fibo

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /fibo

COPY --from=builder /target/release/fibbot /fibo/fibbot

CMD [ "./fibbot" ] 
