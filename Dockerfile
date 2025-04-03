# -----------------
# BUILD STAGE
# -----------------
FROM rust:1.83-slim-bookworm AS builder

# Enable cargo-net fallback (if needed)
ARG CARGO_NET_GIT_FETCH_WITH_CLI=true

# Set working directory with correct path
WORKDIR /build

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
LABEL maintainer="Daniel Stevens" \
      version="0.1.0" \
      description="Tesla Authenticator Service"

# Install runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    libpq5 \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user for security
RUN useradd -m -u 1001 appuser

# Use secure and minimal working dir
WORKDIR /app

# Copy binary
COPY --from=builder \
     /build/target/release/tesla_authenticator /app/tesla_authenticator
    
# Copy configuration file
COPY .env /app/.env

# Set proper permissions
RUN chmod 700 /app && \
    chown -R appuser:appuser /app && \
    chmod +x /app/tesla_authenticator

# Switch to non-root user
USER appuser

# Healthcheck
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Expose port
EXPOSE 8080

# Start the application
CMD ["/app/tesla_authenticator"]
# CMD echo "Starting app..." && /app/tesla_authenticator
