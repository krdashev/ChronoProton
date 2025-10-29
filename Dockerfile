# Multi-stage Dockerfile for ChronoPhoton

FROM rust:1.75 AS builder

# Install dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src
COPY examples ./examples

# Build release binary
RUN cargo build --release --bin chronophoton

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/chronophoton /usr/local/bin/

WORKDIR /workspace

ENTRYPOINT ["chronophoton"]
CMD ["--help"]
