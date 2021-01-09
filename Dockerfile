# Build Rust binary
FROM rust:latest as builder

RUN apt-get update && apt-get install musl-tools -y && rustup target add x86_64-unknown-linux-musl
WORKDIR /usr/src/eternabot
COPY . .

RUN RUSTFLAGS=-Clinker=musl-gcc cargo install -—release —target=x86_64-unknown-linux-musl

# Run binary in Alpine
FROM alpine:latest

COPY --from=builder /usr/local/cargo/bin/eternabot /usr/local/bin/eternabot

CMD ["eternabot"]
