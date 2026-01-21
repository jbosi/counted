#!/usr/bin/env sh
set -e

# Wait for PostgreSQL (max 30 s)
MAX_WAIT=30
elapsed=0
while ! pg_isready -h db -p 5432 -U hcount_user >/dev/null 2>&1; do
  if [ "$elapsed" -ge "$MAX_WAIT" ]; then
    echo "❌ PostgreSQL n'est pas disponible après $MAX_WAIT s"
    exit 1
  fi
  echo "⏳ En attente de PostgreSQL..."
  sleep 2
  elapsed=$((elapsed + 2))
done

# Run migrations
echo "🚀 Application des migrations..."
sqlx migrate run   # ou `diesel migration run`

# Run application
echo "✅ Migrations terminées – lancement du serveur"
exec /usr/local/app/web   # binary file