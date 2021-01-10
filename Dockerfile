FROM rust:1.49 as build

WORKDIR /usr/src/eternabot

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN mkdir src/ && \
    echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/eternabot*
RUN cargo build --release

FROM gcr.io/distroless/cc-debian10

COPY --from=build /usr/src/eternabot/target/release/eternabot /usr/local/bin/eternabot

CMD ["eternabot"]
