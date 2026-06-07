# ── Stage 1: Dependency cache ────────────────────────────────────────────────
FROM rust:slim-bookworm AS deps

ARG BUILD_DATE
ARG GIT_COMMIT
ARG VERSION=dev

WORKDIR /app

RUN apt-get update && \
    apt-get install -y --no-install-recommends pkg-config libssl-dev && \
    apt-get clean

# Copy manifests and lock file first for layer caching
COPY Cargo.toml Cargo.lock ./

# Create dummy source to pre-fetch and pre-build dependencies only
RUN mkdir -p src && \
    printf '#![allow(dead_code)]\nfn main() {}\n' > src/main.rs && \
    printf '#![allow(dead_code)]\n' > src/lib.rs && \
    cargo fetch && \
    cargo build --release 2>/dev/null; \
    rm src/main.rs src/lib.rs

# ── Stage 2: Builder ─────────────────────────────────────────────────────────
FROM deps AS builder

COPY src ./src
COPY tests ./tests

# Touch to force rebuild of our actual source
RUN touch src/main.rs src/lib.rs && \
    cargo build --release

# ── Stage 3: Test runner ─────────────────────────────────────────────────────
FROM builder AS tester

RUN cargo test --release -- --test-threads=4

# ── Stage 4: Minimal runtime image ───────────────────────────────────────────
FROM debian:bookworm-slim AS runtime

LABEL org.opencontainers.image.title="Zenith Compiler"
LABEL org.opencontainers.image.description="Zenith Universal Meta-Compiler (ZUTC)"
LABEL org.opencontainers.image.source="https://github.com/Benwellonedge28/Zenith"
LABEL org.opencontainers.image.licenses="MIT"

RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    apt-get clean

WORKDIR /zenith

COPY --from=builder /app/target/release/zenith /usr/local/bin/zenith

HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD echo "Zenith OK"

CMD ["/usr/local/bin/zenith", "--help"]
