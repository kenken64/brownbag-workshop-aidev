# Stage 1: Build the Angular frontend
FROM node:20-alpine AS frontend-builder

WORKDIR /app/frontend

COPY ../frontend/package.json ../frontend/package-lock.json ./
RUN npm ci

COPY ../frontend/ .
RUN npm run build --configuration=production

# Stage 2: Build the Rust backend
FROM rust:bookworm AS backend-builder

WORKDIR /app/backend

# Install openssl-dev for rusqlite
RUN apt-get update && apt-get install -y libssl-dev pkg-config

COPY backend/Cargo.toml backend/Cargo.lock ./
# This step is to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release

COPY backend/ .
RUN cargo build --release

# Stage 3: Final image
FROM ubuntu:latest

WORKDIR /app

# Install openssl for runtime
RUN apt-get update && apt-get install -y openssl curl && rm -rf /var/lib/apt/lists/*

ENV RUST_BACKTRACE=1

# Copy the built backend executable
COPY --from=backend-builder /app/backend/target/release/backend /app/bin/backend

# Copy the built frontend static files
COPY --from=frontend-builder /app/frontend/dist/frontend/browser /app/static

# Copy the SQLite database (if it exists and is pre-populated)
# If your database is created at runtime, you might not need this.
# COPY backend/todo.db ./backend/todo.db

EXPOSE 8080

HEALTHCHECK --interval=30s --timeout=10s --retries=3 CMD curl --fail http://localhost:8080/api/todos || exit 1

CMD ["/app/bin/backend"]
