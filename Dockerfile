# Cargo Build Stage
FROM rust:latest as cargo-build

RUN apt-get update && \
    apt-get install musl-tools libssh-devel cmake -y && \
    rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/eternabot

COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

RUN mkdir src/ && \
    echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs && \
    RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl && \
    rm -f target/x86_64-unknown-linux-musl/release/deps/eternabot*

COPY . .
RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

# Final Stage
FROM alpine:latest

COPY --from=cargo-build /usr/src/eternabot/target/x86_64-unknown-linux-musl/release/eternabot /usr/local/bin/eternabot

CMD ["eternabot"]
