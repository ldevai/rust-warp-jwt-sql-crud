# Stage 1 - build 
FROM rust:1.45.2-slim as build-deps

LABEL maintainer="leonardo@devai.io"
LABEL version=1.0

WORKDIR /src-root

# COPY Cargo.* ./
# COPY src/ src/
RUN ls -la
RUN pwd
RUN apt-get update && apt-get install -y build-essential libssl-dev pkg-config clang llvm-dev libclang-dev wait-for-it
RUN cargo install systemfd cargo-watch
ENTRYPOINT ["wait-for-it", "db:5432", "--", "./scripts/run_dev.sh"]
