# syntax=docker/dockerfile:1

FROM rust:latest as builder

# Set PATH for cargo binaries
ENV PATH="/usr/local/cargo/bin:${PATH}"

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Install sqlx-cli for migrations and query checking
RUN cargo install sqlx-cli --no-default-features --features "postgres,sqlite"

# Copy workspace files
COPY Cargo.toml ./
COPY backend/Cargo.toml ./backend/
COPY shared/Cargo.toml ./shared/
COPY frontend/Cargo.toml ./frontend/

# Copy source code
COPY shared/ ./shared/
COPY backend/ ./backend/
COPY frontend/ ./frontend/

# Set SQLx to offline mode for Railway deployment (no database access during build)
ENV SQLX_OFFLINE=true

# Build the application
RUN cargo build --release --bin work-session-tracker-backend

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/work-session-tracker-backend /app/backend

# Copy migrations
COPY backend/migrations/ ./migrations/

# Railway uses PORT environment variable
EXPOSE $PORT

# Add healthcheck
HEALTHCHECK --interval=30s --timeout=30s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:${PORT:-8080}/api/sessions || exit 1

CMD ["./backend"]