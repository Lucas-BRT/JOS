# --- Builder Stage ---
FROM rust:1.90-slim as builder

RUN apt-get update && apt-get install -y musl-tools musl-dev

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main(){}" > src/main.rs && \
    cargo build --release --target=x86_64-unknown-linux-musl && \
    rm -rf src

COPY src ./src
COPY migrations ./migrations
COPY .sqlx ./.sqlx

RUN touch src/main.rs && cargo build --release --target=x86_64-unknown-linux-musl

# --- Final Stage ---
FROM scratch

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/jos /jos

EXPOSE 3000

CMD ["/jos"]