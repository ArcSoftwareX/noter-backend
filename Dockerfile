# FROM lukemathwalker/cargo-chef:latest-rust-1.73.0 AS chef
# WORKDIR /app

# FROM chef AS planner
# COPY . .
# RUN cargo chef prepare --recipe-path recipe.json

# FROM chef AS builder
# COPY --from=planner /app/recipe.json recipe.json
# # Build dependencies - this is the caching Docker layer!
# RUN cargo chef cook --recipe-path recipe.json --release

# COPY . .
# RUN cargo build --release

# FROM rust:1.73-slim AS template-rust
# COPY --from=builder /app/target/release/backend /usr/local/bin
# ENTRYPOINT ["/usr/local/bin/backend"]

FROM rust:1.73.0 as build

RUN USER=root cargo new --bin backend
WORKDIR /backend

COPY Cargo.toml Cargo.lock ./

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/backend*
RUN cargo build --release


FROM debian:bookworm-slim as runtime

COPY --from=build /backend/target/release/backend .

ENTRYPOINT [ "/backend" ]
EXPOSE 8080