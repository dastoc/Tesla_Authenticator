# -----------------
# BUILD STAGE
# -----------------
FROM rust:latest AS builder

# Enable cargo-net fallback (if needed)
ARG CARGO_NET_GIT_FETCH_WITH_CLI=true

# Set working directory with correct path
WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs \
    && cargo build --release \
    && rm -rf src

# Copy source code
COPY src/ ./src/
COPY migrations/ ./migrations/

# Build with optimizations
RUN cargo build --release --locked \
    && strip target/release/tesla_authenticator

# -----------------
# RUNTIME STAGE
# -----------------
FROM debian:bookworm-slim AS runtime

# Metadata labels
LABEL maintainer="Daniel Stevens"
LABEL version="0.1.0"
LABEL description="Tesla Authenticator Service"

# Install runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    libpq5 \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user for security
RUN useradd -m -r appuser -u 1001

# Use secure and minimal working dir
WORKDIR /app
USER appuser

# COpy binary with ownership
COPY --from=builder --chown=appuser:appuser \
    /app/target/release/tesla_authenticator .

# Copy configuration with ownership
COPY --chown=appuser::appuser .env.example .env

# Healthcheck
HEALTHCHECK --interval=30s --timeout=3s \
    CMD curl -f http://localhost::8080/health || exit 1

# Expose port
EXPOSE 8080/tcp

# Set runtime environment 
ENV RUST_LOG=info \
    PORT=8080

# Start the application
CMD ["./tesla_authenticator"]
