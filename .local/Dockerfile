# Dockerfile for local development
FROM rust:1.90-slim as development

RUN apt-get update && apt-get install -y netcat-openbsd curl

RUN cargo install cargo-watch && \
    cargo install sqlx-cli --no-default-features --features postgres && cargo install cargo-nextest

WORKDIR /app

COPY ./.local/entrypoint.sh /usr/local/bin/entrypoint.sh
RUN chmod +x /usr/local/bin/entrypoint.sh

ENTRYPOINT ["entrypoint.sh"]

CMD ["cargo", "watch", "-q", "-c", "-w", "src/", "-x", "run --bin jos"]
