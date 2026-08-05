# ── Zamani Language Server (zamani-lsp) ──────────────────────────────────────
# Status: WORKING. Builds with `--features lsp`. Real stdio LSP server backed
# by the actual lexer/parser (diagnostics reflect genuine parse errors, not a
# stub). Speaks LSP-framed JSON-RPC over stdin/stdout, so this image is meant
# to be exec'd by an editor/client (e.g. `docker run -i`), not left idling.
FROM rust:slim-bookworm AS builder
ARG BUILD_DATE
ARG GIT_COMMIT
ARG VERSION=dev
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends pkg-config libssl-dev && apt-get clean

COPY Cargo.toml Cargo.lock ./
COPY benches ./benches
COPY src ./src
RUN cargo build --release --features lsp --bin zamani-lsp

FROM debian:bookworm-slim AS runtime
LABEL org.opencontainers.image.title="Zamani Language Server"
LABEL org.opencontainers.image.description="Zamani LSP (real lexer/parser diagnostics over stdio)"
LABEL org.opencontainers.image.source="https://github.com/Benwellonedge28/Zamani"
LABEL org.opencontainers.image.version="${VERSION}"
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && apt-get clean
WORKDIR /zamani
COPY --from=builder /app/target/release/zamani-lsp /usr/local/bin/zamani-lsp
ENTRYPOINT ["/usr/local/bin/zamani-lsp"]
