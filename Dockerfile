FROM rust:1.85-alpine AS builder

# Install system dependencies
RUN apk add --no-cache build-base jpeg-dev libpng-dev

WORKDIR /app

# Copy dependency files
COPY Cargo.toml Cargo.lock ./
COPY jos-cli/Cargo.toml ./jos-cli/

# Create dummy source files for dependency resolution
RUN mkdir -p src jos-cli/src && \
    echo "fn main() {}" > src/main.rs && \
    echo "fn main() {}" > jos-cli/src/main.rs

# Build dependencies only (this layer will be cached)
RUN cargo build --release

# Copy source code
COPY . .

# Build the application
RUN cargo build --release

# Production stage
FROM alpine:latest

# Install runtime dependencies
RUN apk add --no-cache ca-certificates postgresql-client

# Create non-root user
RUN addgroup -g 1001 -S jos && \
    adduser -S jos -u 1001

# Copy binary from builder
COPY --from=builder /app/target/release/jos /usr/local/bin/

# Switch to non-root user
USER jos

# Expose port
EXPOSE 3000

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://localhost:3000/health || exit 1

# Run the application
CMD ["jos"]
