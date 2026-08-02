FROM rust:1.97.0-alpine3.22 AS builder

WORKDIR /usr/src/app

RUN apk add libc-dev openssl-dev openssl-libs-static

COPY Cargo.toml Cargo.lock ./
COPY templates templates
COPY fonts fonts
COPY src src
COPY data/lang2hex.json data/lang2hex.json
RUN cargo build --release

FROM alpine:latest

WORKDIR /usr/local/bin

COPY --from=builder /usr/src/app/target/release/stats-cards stats-cards
COPY assets assets
ENV SERVICE_HOST=0.0.0.0

CMD ["./stats-cards"]