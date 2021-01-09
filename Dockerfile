# Cargo Build Stage
FROM rust:latest as cargo-build

WORKDIR /usr/src/eternabot

COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

RUN mkdir src/ && \
    echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs && \
    cargo build --release && \
    rm -f target/release/deps/eternabot*

COPY . .

RUN cargo build --release
RUN cargo install --path .

# Final Stage
FROM alpine:latest

COPY --from=cargo-build /usr/local/cargo/bin/eternabot /usr/local/bin/eternabot

CMD ["eternabot"]
