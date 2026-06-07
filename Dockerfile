# ── Stage 1: Chef / planner ───────────────────────────────────────────────────
FROM rust:slim-bookworm AS deps

ARG BUILD_DATE
ARG GIT_COMMIT
ARG VERSION=dev

WORKDIR /app

RUN apt-get update && \
    apt-get install -y --no-install-recommends pkg-config libssl-dev && \
    apt-get clean

# Copy manifests and lock file for reproducible dependency caching
COPY Cargo.toml Cargo.lock ./

# Pre-fetch all dependencies using a dummy binary and library
# This layer is only rebuilt when Cargo.toml/Cargo.lock changes
RUN mkdir -p src && \
    printf 'fn main() {}\n' > src/main.rs && \
    printf 'pub fn _placeholder() {}\n' > src/lib.rs && \
    cargo build --release 2>&1; \
    rm src/main.rs src/lib.rs

# ── Stage 2: Full source build ───────────────────────────────────────────────
FROM deps AS builder

# Copy entire source tree (including .zn preserved files)
COPY src ./src
COPY tests ./tests

# Touch to invalidate incremental cache from dummy build, then build for real
RUN touch src/main.rs src/lib.rs && cargo build --release

# ── Stage 3: Minimal runtime ─────────────────────────────────────────────────
FROM debian:bookworm-slim AS runtime

LABEL org.opencontainers.image.title="Zenith Compiler"
LABEL org.opencontainers.image.description="Zenith Universal Meta-Compiler (ZUTC)"
LABEL org.opencontainers.image.source="https://github.com/Benwellonedge28/Zenith"
LABEL org.opencontainers.image.licenses="MIT"
LABEL org.opencontainers.image.version="${VERSION}"
LABEL org.opencontainers.image.created="${BUILD_DATE}"
LABEL org.opencontainers.image.revision="${GIT_COMMIT}"

RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    apt-get clean

WORKDIR /zenith

COPY --from=builder /app/target/release/zenith /usr/local/bin/zenith

HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD echo "Zenith OK"

CMD ["/usr/local/bin/zenith", "--help"]
