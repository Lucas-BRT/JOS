FROM rust:1.85-alpine AS builder

RUN apk add --no-cache build-base jpeg-dev libpng-dev

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY jos-cli/Cargo.toml ./jos-cli/

RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN mkdir jos-cli/src && echo "fn main() {}" > jos-cli/src/main.rs

RUN cargo build --release --locked

COPY src ./src
COPY jos-cli/src ./jos-cli/src
COPY migrations ./migrations

RUN rm -f target/release/deps/jos* && cargo build --release --locked

FROM alpine:latest

RUN apk add --no-cache ca-certificates

COPY --from=builder /app/target/release/jos /usr/local/bin/

EXPOSE 3000

CMD ["jos"]
