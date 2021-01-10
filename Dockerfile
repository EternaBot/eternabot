FROM rust:latest as build
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/eternabot
COPY Cargo.toml Cargo.lock ./
RUN mkdir src/
RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs
RUN cargo build --release
RUN rm ./target/release/deps/eternabot*

COPY . .

RUN cargo build --release

FROM gcr.io/distroless/cc-debian10

COPY --from=build /usr/src/eternabot/target/release/eternabot /usr/local/bin/eternabot

CMD ["eternabot"]
