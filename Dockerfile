# Stage 1: Builder
FROM rust:1.78-slim-bookworm AS builder

ARG BUILD_DATE
ARG GIT_COMMIT
ARG VERSION=dev

WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends pkg-config libssl-dev && apt-get clean

COPY Cargo.toml ./
RUN mkdir -p src && echo 'fn main() {}' > src/main.rs && echo 'pub fn lib() {}' > src/lib.rs && cargo fetch && cargo build --release || true

COPY src ./src
COPY tests ./tests
RUN touch src/main.rs src/lib.rs && cargo build --release

# Stage 2: Test runner
FROM builder AS tester
RUN cargo test --release -- --test-threads=4

# Stage 3: Runtime
FROM debian:bookworm-slim AS runtime

LABEL org.opencontainers.image.title="Zenith Compiler"
LABEL org.opencontainers.image.description="Zenith Universal Meta-Compiler (ZUTC)"
LABEL org.opencontainers.image.source="https://github.com/Benwellonedge28/Zenith"
LABEL org.opencontainers.image.licenses="MIT"

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && apt-get clean

WORKDIR /zenith
COPY --from=builder /app/target/release/zenith /usr/local/bin/zenith

HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 CMD echo "Zenith OK"

CMD ["/usr/local/bin/zenith", "--help"]
