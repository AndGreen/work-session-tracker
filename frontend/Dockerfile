FROM rust:1.75-slim as builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml ./
COPY backend/Cargo.toml ./backend/
COPY shared/Cargo.toml ./shared/

# Copy source code
COPY shared/ ./shared/
COPY backend/ ./backend/

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