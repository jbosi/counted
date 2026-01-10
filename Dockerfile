FROM rust:1 AS chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .

# Install `dx`
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall dioxus-cli --root /.cargo -y --force
ENV PATH="/.cargo/bin:$PATH"

# Create the final bundle folder. Bundle with release build profile to enable optimizations.
RUN dx bundle --web --release --package web

FROM chef AS runtime
COPY --from=builder /app/target/dx/web/release/web/ /usr/local/app

# Copy migration folder & run
COPY --from=builder /app/migrations /usr/local/app/migrations
RUN apt-get update && apt-get install -y --no-install-recommends \
	postgresql-client && \
	rm -rf /var/lib/apt/lists/*
RUN cargo install sqlx-cli --locked --root /usr/local/cargo && \
	ln -s /usr/local/cargo/bin/sqlx /usr/local/bin/sqlx

ENV PATH="/usr/local/cargo/bin:${PATH}"

# set our port and make sure to listen for all connections
ENV PORT=8080
ENV IP=0.0.0.0
ENV DATABASE_URL=postgres://hcount_user:supersecret@db:5432/hcount

# expose the port 8080
EXPOSE 8080

WORKDIR /usr/local/app

# ----- Copy migration script et make it runable -----
COPY runMigrationsAndBinary.sh /runMigrationsAndBinary.sh
RUN chmod +x /runMigrationsAndBinary.sh

# Run migration script + run executable
ENTRYPOINT ["/runMigrationsAndBinary.sh"]

