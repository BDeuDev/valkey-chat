#!/bin/sh
set -e

echo "⏳ Waiting for Valkey at $VALKEY_HOST:$VALKEY_PORT..."
until nc -z $VALKEY_HOST $VALKEY_PORT; do
  sleep 1
done
echo "✅ Valkey ready!"

# Opcional: migraciones
# sqlx migrate run

echo "🚀 Starting valkey-chat..."
exec ./valkey-chat
