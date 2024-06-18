FROM rust:1.79.0 AS build
LABEL authors="treize"

RUN USER=root cargo new --bin webapi
WORKDIR /webapi

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/webapi*
RUN cargo build --release

FROM debian:stable-slim
LABEL authors="treize"

WORKDIR /app

COPY --from=build /webapi/target/release/webapi .

CMD ["./webapi"]