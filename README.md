# Development

## Build libraries images

Generate image for dioxus + sqlx-cli used for the main image

```bash
cd .\libraries\docker\tools\
docker build -t counted-tools -f Dockerfile .
```

## Update sqlx schema

Run every time the db queries / schema change

```bash
cargo sqlx prepare --workspace -- --all-features
```

# Deploy

To check status and ports

```bash
docker-compose ps
```

Get docker db (ex: projects)

```bash
docker-compose exec db psql -U hcount_user -d hcount -c "SELECT id, name, currency, description FROM projects;"
```
