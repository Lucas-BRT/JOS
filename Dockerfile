# ----------------- Development Stage -----------------
FROM rust:1.90-slim as development

# Install development tools
RUN apt-get update && apt-get install -y netcat-openbsd curl && rm -rf /var/lib/apt/lists/*
RUN cargo install cargo-watch && \
    cargo install sqlx-cli --no-default-features --features postgres && \
    cargo install cargo-nextest

WORKDIR /app

# Copy entrypoint for local setup
COPY ./.local/entrypoint.sh /usr/local/bin/entrypoint.sh
RUN chmod +x /usr/local/bin/entrypoint.sh

ENTRYPOINT ["entrypoint.sh"]
CMD ["cargo", "watch", "-q", "-c", "-w", "api/src", "-w", "application/src", "-w", "domain/src", "-w", "infrastructure/src", "-w", "shared/src", "-w", "src", "-x", "run --bin jos"]

# ----------------- Builder Stage -----------------
FROM rust:1.90-slim as builder

WORKDIR /app

# Install sqlx-cli for query preparation
RUN cargo install sqlx-cli --no-default-features --features postgres

# Copy the entire project
COPY . .

# Prepare sqlx queries
RUN cargo sqlx prepare --workspace

# Build the application binary
RUN cargo build --release --bin jos

# ----------------- Production Stage -----------------
FROM ubuntu:22.04 as production

# Install runtime dependencies
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/jos .

# Make binary executable
RUN chmod +x ./jos

EXPOSE 3000

# Run the application
CMD ["./jos"]
