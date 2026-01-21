FROM rust:1 AS chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM lewimbes/dioxus:latest AS builder
RUN cargo install cargo-chef
WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .

# Create the final bundle folder. Bundle with release build profile to enable optimizations.
RUN dx bundle --web --release --package web

FROM nxyt/sqlx-cli AS sqlx

FROM debian:bookworm-slim AS runtime
COPY --from=sqlx /bin/sqlx /usr/local/bin/sqlx
COPY --from=builder /app/target/dx/web/release/web/ /usr/local/app

# Copy migration folder
COPY --from=builder /app/migrations /usr/local/app/migrations

# Install CA certificates and PostgreSQL client
RUN apt-get update && apt-get install -y --no-install-recommends \
	ca-certificates \
	postgresql-client && \
	rm -rf /var/lib/apt/lists/*

# Set our port and make sure to listen for all connections
ENV PORT=8080
ENV IP=0.0.0.0
ENV DATABASE_URL=postgres://hcount_user:supersecret@db:5432/hcount

# Expose the port 8080
EXPOSE 8080

WORKDIR /usr/local/app

# Run migrations then start the application
CMD ["sh", "-c", "sqlx migrate run && ./web"]
