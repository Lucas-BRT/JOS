FROM rust:1.85-alpine AS builder

RUN apk add --no-cache build-base jpeg-dev libpng-dev

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release --locked

COPY src ./src
COPY migrations ./migrations

RUN rm -f target/release/deps/jos* && cargo build --release --locked


FROM alpine:latest

COPY --from=builder /app/target/release/jos /usr/local/bin/

EXPOSE 3000

CMD ["jos"]
