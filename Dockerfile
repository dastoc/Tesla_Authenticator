# ----------------- #
#     BASE STAGE    #
# ----------------- #
# This stage sets up the base image with Rust and essential build tools.
FROM rust:1.83 AS base

# Install cargo-chef for caching dependencies and sccache for build caching
RUN cargo install --locked cargo-chef sccache

# Set environment variables for Rust compiler wrapper to use sccache
ENV RUSTC_WRAPPER=sccache \
    SCCACHE_DIR=/sccache

# ----------------- #
#   PLANNER STAGE   #
# ----------------- #
# This stage is used to analyze dependencies and generate a build plan using cargo-chef
FROM base AS planner

# Set working directory for planner
WORKDIR /plan

# Copy entire project files (including Cargo.toml & Cargo.lock)
COPY . .

# Generate recipe.json which contains dependency graph
RUN cargo chef prepare --recipe-path recipe.json

# ----------------- #
#   BUILDER STAGE   #
# ----------------- #
# This stage uses the plan from the planner stage to cache dependencies and build the binary
FROM base AS builder

# Set working directory for builder
WORKDIR /build

# Copy the dependency recipe file from planner stage
COPY --from=planner /plan/recipe.json recipe.json

# Pre-build dependencies using the recipe, caching cargo registry/git and sccache
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo chef cook --release --recipe-path recipe.json

# Copy the actual source code for building final binary
COPY . .

# Build the project in release mode and strip the resulting binary to reduce size
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo build --release --locked \
    && strip target/release/tesla_authenticator

# ----------------- #
#   RUNTIME STAGE   #
# ----------------- #
# This final stage contains only the compiled binary and its runtime dependencies
FROM debian:bookworm-slim AS runtime

# Metadata labels image identification
LABEL maintainer="Daniel Stevens" \
      version="0.1.0" \
      description="Tesla Authenticator Service"

# Install only the necessary runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    libpq5 \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user for security
RUN useradd -m -u 1001 appuser

# Set working directory
WORKDIR /app

# Copy the built binary from the builder stage
COPY --from=builder \
     /build/target/release/tesla_authenticator /app/tesla_authenticator
    
# Copy the environment configuration file
COPY .env /app/.env

# Set permissions for security and execution
RUN chmod 700 /app && \
    chown -R appuser:appuser /app && \
    chmod +x /app/tesla_authenticator

# Use non-root user for security best practices
USER appuser

# Define a health check endpoint
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:0.0.0.0/health || exit 1

# Expose application port
EXPOSE 8080

# Start the application
CMD ["/app/tesla_authenticator"]
