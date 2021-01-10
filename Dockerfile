FROM rust:1.49 as build

RUN USER=root cargo new --bin eternabot
WORKDIR /usr/src/eternabot

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/eternabot*
RUN cargo build --release

FROM gcr.io/distroless/cc-debian10

COPY --from=build /usr/src/eternabot/target/release/eternabot /usr/local/bin/eternabot

CMD ["eternabot"]
