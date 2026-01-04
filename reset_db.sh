#!/bin/bash

# chmod +x reset_db.sh


# Exit on error
set -e

# Load .env file (containing POSTGRES_USER, POSTGRES_PASSWORD, POSTGRES_DATABASE, etc.)
source .env

echo "Terminating all connections to database: $POSTGRES_DATABASE"
psql -h 127.0.0.1 -U "$POSTGRES_USER" -d postgres -c "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = '$POSTGRES_DATABASE';"

echo "Dropping database: $POSTGRES_DATABASE"
psql -h 127.0.0.1 -U "$POSTGRES_USER" -d postgres -c "DROP DATABASE IF EXISTS $POSTGRES_DATABASE;"

echo "Creating database: $POSTGRES_DATABASE"
psql -h 127.0.0.1 -U "$POSTGRES_USER" -d postgres -c "CREATE DATABASE $POSTGRES_DATABASE;"

echo "Running migrations with sqlx"
sqlx migrate run

echo "Done ✔️"