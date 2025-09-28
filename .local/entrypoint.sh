#!/bin/sh
set -e

echo "Waiting PostgreSQL..."
while ! nc -z db 5432; do
  sleep 0.1
done
echo "PostgreSQL initialized successfully!"

exec "$@"