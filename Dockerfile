# ── Stage 1: Build ─────────────────────────────────────────────
FROM rust:1.86 AS builder

WORKDIR /app

# aws-lc-sys (via rustls) builds from source and needs CMake + Perl;
# ring needs pkg-config. None of these ship with the stock rust image.
RUN apt-get update \
    && apt-get install -y --no-install-recommends cmake perl pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Copy the workspace manifest + sources
COPY Cargo.toml Cargo.lock ./
COPY crates crates
# COPY migration migration
COPY src src

RUN cargo build --release --bin Rust

# ── Stage 2: Runtime ───────────────────────────────────────────
FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/Rust /app/hulupay

EXPOSE 5000

ENV RUST_LOG=info

CMD ["/app/hulupay"]
