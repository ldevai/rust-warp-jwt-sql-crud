# Stage 1 - build 
FROM rust:1.45.2-slim as build-deps

LABEL maintainer="leonardo@devai.io"
LABEL version=1.0

WORKDIR /app

COPY Cargo.* ./
COPY src/ src/

RUN apt-get update && apt-get install -y build-essential libssl-dev pkg-config clang llvm-dev libclang-dev wait-for-it
RUN cargo install systemfd cargo-watch

# Stage 2 - deploy 
FROM debian:buster-slim

LABEL maintainer="leonardo.devai@gmail.com"
LABEL version=1.0

WORKDIR /usr/src/web-app

RUN apt-get update && apt-get install -y libssl-dev ca-certificates

COPY --from=build-deps /usr/local/cargo/bin/rust-warp-jwt-sql-crud /usr/local/bin/rust-warp-jwt-sql-crud

ENV RUST_LOG=info
CMD ["rust-warp-jwt-sql-crud"]
