#!/bin/sh
set -e

echo "Waiting for PostgreSQL and running migrations..."
until diesel migration run; do
  echo "PostgreSQL is unavailable - sleeping"
  sleep 2
done

echo "Starting application..."
exec cargo watch -x run
