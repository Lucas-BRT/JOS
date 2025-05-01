# Stage 1: build with musl
FROM rust:1.85 as builder

# Add target musl
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app

# Copy the code and build the final binary
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

# Stage 2: runtime with alpine
FROM alpine:3.18

# Install PostgreSQL dependencies
RUN apk add libpq

WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/JOS ./JOS

# Export port
EXPOSE 3000

# Executar o binário
CMD ["./JOS"]
