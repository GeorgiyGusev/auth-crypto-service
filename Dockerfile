# Multi-stage build
FROM rust:1.91-alpine AS planner

# Устанавливаем необходимые системные пакеты
RUN apk add --no-cache musl-dev gcc curl

WORKDIR /app
RUN cargo install cargo-chef
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo chef prepare --recipe-path recipe.json

FROM rust:1.91-alpine AS cacher

# Устанавливаем необходимые системные пакеты
RUN apk add --no-cache musl-dev gcc curl

WORKDIR /app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust:1.91-alpine AS builder 

# Устанавливаем необходимые системные пакеты
RUN apk add --no-cache musl-dev gcc curl

WORKDIR /app
COPY . .
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release && \
    strip target/release/app

# Финальный образ
FROM gcr.io/distroless/cc-debian12

WORKDIR /app
COPY --from=builder /app/target/release/app ./

USER nonroot:nonroot

EXPOSE 3000
CMD ["./app"]