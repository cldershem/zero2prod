# FROM lukemathwalker/cargo-chef AS planner
# WORKDIR app
# COPY . .
# RUN cargo chef prepare  --recipe-path recipe.json

# FROM lukemathwalker/cargo-chef as cacher
# WORKDIR app
# COPY --from=planner /app/recipe.json recipe.json
# RUN cargo chef cook --release --recipe-path recipe.json

# FROM rust:1.50 AS builder
# WORKDIR app
# COPY --from=cacher /app/target target
# COPY --from=cacher /usr/local/cargo /usr/local/cargo
# COPY . .
# ENV SQLX_OFFLINE true
# RUN cargo build --release --bin zero2prod

FROM rust:1.56 AS builder
WORKDIR /app
RUN apt update && apt install lld clang -y
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release --bin zero2prod

FROM debian:buster-slim AS runtime
WORKDIR app
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/zero2prod zero2prod
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./zero2prod"]
