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

FROM chef AS runtime
COPY --from=builder /app/target/dx/web/release/web/ /usr/local/app
COPY --from=builder /app/migrations /usr/local/app/migrations
COPY --from=builder /usr/local/cargo/bin/sqlx /usr/local/bin/sqlx

# set our port and make sure to listen for all connections
ENV PORT=8080
ENV IP=0.0.0.0
ENV DATABASE_URL=postgres://hcount_user:supersecret@db:5432/hcount

# expose the port 8080
EXPOSE 8080

WORKDIR /usr/local/app

# ----- Copy migration script et make it runable -----
COPY runMigrationsAndBinary.sh /runMigrationsAndBinary.sh
# Fix line endings (convert CRLF to LF for Windows compatibility)
RUN sed -i 's/\r$//' /runMigrationsAndBinary.sh
RUN chmod +x /runMigrationsAndBinary.sh

# Run migration script + run executable
ENTRYPOINT ["/runMigrationsAndBinary.sh"]

