FROM rust:latest AS builder

WORKDIR /app

RUN rustup toolchain install nightly --profile minimal \
    && rustup default nightly \
    && rustup target add wasm32-unknown-unknown \
    && cargo install cargo-leptos --locked

COPY . .

RUN cargo leptos build --release

FROM debian:trixie-slim

RUN apt-get update \
    && apt-get install --no-install-recommends -y ca-certificates libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/datingapp /app/target/release/datingapp
COPY --from=builder /app/target/site /app/target/site

ENV LEPTOS_SITE_ROOT=target/site

CMD ["/app/target/release/datingapp"]
