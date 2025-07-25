# syntax=docker/dockerfile:1

FROM rust:latest AS build

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Install Trunk for building the Yew app and the wasm target
RUN cargo install trunk --locked && \
    rustup target add wasm32-unknown-unknown

WORKDIR /app

# Copy workspace manifests first for efficient Docker caching
COPY Cargo.toml ./
COPY frontend/Cargo.toml ./frontend/
COPY shared/Cargo.toml ./shared/

# Copy actual source code
COPY frontend ./frontend
COPY shared ./shared

# Build the frontend (output will be written to /dist)
RUN cd frontend && trunk build --release --dist /dist

# ---------------- Runtime image ----------------
FROM nginx:1.25-alpine AS runtime

# Copy the build artefacts from the previous stage
COPY --from=build /dist /usr/share/nginx/html

# Use Railway-optimized nginx configuration for production
COPY frontend/nginx-railway.conf /etc/nginx/nginx.conf

# Create a script to substitute environment variables in nginx config
RUN echo '#!/bin/sh' > /docker-entrypoint.sh && \
    echo 'envsubst < /etc/nginx/nginx.conf > /tmp/nginx.conf' >> /docker-entrypoint.sh && \
    echo 'mv /tmp/nginx.conf /etc/nginx/nginx.conf' >> /docker-entrypoint.sh && \
    echo 'exec nginx -g "daemon off;"' >> /docker-entrypoint.sh && \
    chmod +x /docker-entrypoint.sh

EXPOSE 8000

CMD ["/docker-entrypoint.sh"]