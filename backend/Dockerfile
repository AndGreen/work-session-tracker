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

# Set DATABASE_URL for sqlx compile-time checks
ENV DATABASE_URL postgres://postgres:password@postgres:5432/work_tracker

# Build the application
RUN cargo build --release --bin work-session-tracker-backend

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/work-session-tracker-backend /app/backend

# Copy migrations
COPY backend/migrations/ ./migrations/

EXPOSE 8080

CMD ["./backend"]