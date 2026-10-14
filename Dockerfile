# =============================================================================
#  Dockerfile : PerfReport (Rust)  |  IS2083 Lab 3
#  Multi-stage build. Stage 1 compiles the Rust binary (and downloads the
#  sysinfo crate). Stage 2 is a small runtime image with just the binary.
# =============================================================================

# ---- Stage 1: build ---------------------------------------------------------
FROM rust:1.99-slim-bookworm AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release --locked

# ---- Stage 2: runtime -------------------------------------------------------
FROM debian:bookworm-slim
COPY --from=builder /app/target/release/perfreport /usr/local/bin/perfreport
ENTRYPOINT ["/usr/local/bin/perfreport"]
