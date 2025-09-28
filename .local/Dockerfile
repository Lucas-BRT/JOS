# Dockerfile for local development
FROM rust:1.90-slim as development

RUN apt-get update && apt-get install -y netcat-openbsd

RUN cargo install cargo-watch && \
    cargo install sqlx-cli --no-default-features --features postgres

WORKDIR /app

COPY ./.local/entrypoint.sh /usr/local/bin/entrypoint.sh
RUN chmod +x /usr/local/bin/entrypoint.sh

ENTRYPOINT ["entrypoint.sh"]

CMD ["cargo", "watch", "-q", "-c", "-w", "src/", "-x", "run --bin jos"]