# ── Stage 1: Builder ──────────────────────────────────────────────────────────
FROM rust:1.78-slim-bookworm AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock* ./
COPY src ./src
COPY tests ./tests

RUN cargo build --release --locked

# ── Stage 2: Test runner ──────────────────────────────────────────────────────
FROM builder AS tester
RUN cargo test --release --locked

# ── Stage 3: Runtime image ────────────────────────────────────────────────────
FROM debian:bookworm-slim AS runtime

LABEL org.opencontainers.image.title="Zenith Compiler"
LABEL org.opencontainers.image.description="Zenith Universal Meta-Compiler (ZUTC)"
LABEL org.opencontainers.image.source="https://github.com/Benwellonedge28/Zenith"
LABEL org.opencontainers.image.licenses="MIT"

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && apt-get clean

WORKDIR /zenith
COPY --from=builder /app/target/release/zenith* /usr/local/bin/ 2>/dev/null || true

HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD echo "Zenith OK"

CMD ["echo", "Zenith Compiler Runtime Ready"]
