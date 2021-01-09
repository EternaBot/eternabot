FROM rust:latest as build
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/eternabot
COPY . .

RUN cargo install --path .

FROM gcr.io/distroless/cc-debian10

COPY --from=build /usr/local/cargo/bin/eternabot /usr/local/bin/eternabot

CMD ["eternabot"]
