FROM jbosi/counted-tools AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Use cache mounts to speed up cargo builds
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/app/target \
    cargo chef cook --release --recipe-path recipe.json
COPY . .

# Create the final bundle folder. Bundle with release build profile to enable optimizations.
# Note: We don't cache /app/target here because we need to copy the build output in the next stage
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    dx bundle --web --release --package web

FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates libssl3 postgresql-client && \
    rm -rf /var/lib/apt/lists/* && \
    adduser --disabled-password --gecos "" --uid 10001 appuser

COPY --from=builder /app/target/dx/web/release/web/ /usr/local/app
COPY --from=builder /app/migrations /usr/local/app/migrations
COPY --from=builder /usr/local/cargo/bin/sqlx /usr/local/bin/sqlx
COPY runMigrationsAndBinary.sh /runMigrationsAndBinary.sh
# Fix line endings (convert CRLF to LF for Windows compatibility) and ensure executables are runnable by non-root user
RUN sed -i 's/\r$//' /runMigrationsAndBinary.sh && \
    chmod +x /runMigrationsAndBinary.sh /usr/local/app/web

ENV PORT=8080
ENV IP=0.0.0.0
EXPOSE 8080
WORKDIR /usr/local/app
USER appuser

ENTRYPOINT ["/runMigrationsAndBinary.sh"]
