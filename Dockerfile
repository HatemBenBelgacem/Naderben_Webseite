# syntax=docker/dockerfile:1

# ───────────── Build-Stage ─────────────
FROM rust:1.96-bookworm AS builder

# Tailwind-Version für reproduzierbare Builds fixieren (cargo-leptos lädt das Binary)
ENV LEPTOS_TAILWIND_VERSION=v4.2.1

# cargo-leptos als fertiges Binary installieren (schneller als kompilieren)
RUN curl -L --proto '=https' --tlsv1.2 -sSf \
      https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash \
 && cargo binstall cargo-leptos --version 0.3.10 -y --locked \
 && rustup target add wasm32-unknown-unknown

WORKDIR /app
COPY . .

RUN cargo leptos build --release \
 && mkdir -p /out \
 && cp target/release/naderben /out/naderben \
 && cp -r target/site /out/site

# ───────────── Runtime-Stage ─────────────
FROM debian:bookworm-slim AS runtime

RUN apt-get update \
 && apt-get install -y --no-install-recommends ca-certificates \
 && rm -rf /var/lib/apt/lists/* \
 && useradd --system --uid 10001 app

WORKDIR /app
COPY --from=builder /out/naderben /app/naderben
COPY --from=builder /out/site /app/site
COPY --from=builder /app/Cargo.toml /app/Cargo.toml

ENV RUST_LOG="info" \
    LEPTOS_OUTPUT_NAME="naderben" \
    LEPTOS_SITE_ROOT="site" \
    LEPTOS_SITE_PKG_DIR="pkg" \
    LEPTOS_SITE_ADDR="0.0.0.0:8080" \
    LEPTOS_RELOAD_PORT="3001" \
    LEPTOS_ENV="PROD"

# Railway setzt $PORT – main.rs bindet automatisch an 0.0.0.0:$PORT
EXPOSE 8080
USER app
CMD ["/app/naderben"]
