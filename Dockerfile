# FROM lewimbes/dioxus

# # Install necessary build dependencies
# # RUN apk add --no-cache musl-dev gcc openssl-dev

# WORKDIR /app

# # Copy the Cargo.toml and Cargo.lock files to build the dependency cache
# COPY Cargo.toml ./
# COPY Cargo.lock ./

# # Now copy the actual source code
# # COPY ./src ./src

# # Build the Rust application in release mode
# RUN cargo build --release

# # Ensure the binary is executable
# RUN chmod +x ./target/release/todo_rust_react_chakra_ui_example

# # Set environment variables (if needed)
# ENV APP_HOST=127.0.0.1
# ENV APP_PORT=8080

# # Expose the server port
# EXPOSE 8080

# # Start the application (assuming the binary is in `./target/release`)
# CMD ["./target/release/todo_rust_react_chakra_ui_example"]

FROM rust:1-slim AS chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json


# Limit parallel jobs, enable a modest swap file (1 GiB)
ENV CARGO_BUILD_JOBS=1
# disables incremental compilation, saves RAM
ENV CARGO_INCREMENTAL=0

# Use BuildKit cache mounts for Cargo registries
RUN --mount=type=cache,target=/usr/local/cargo/registry \
	--mount=type=cache,target=/usr/local/cargo/git \
	cargo chef cook --release --recipe-path recipe.json

# Install `dx` pre-compiled binary
RUN apt-get update && apt-get install -y wget && \
	wget https://github.com/DioxusLabs/dioxus/releases/latest/download/dx-x86_64-unknown-linux-gnu.tar.gz && \
	tar -xzf dx-x86_64-unknown-linux-gnu.tar.gz && \
	mv dx /usr/local/bin/ && \
	chmod +x /usr/local/bin/dx && \
	rm dx-x86_64-unknown-linux-gnu.tar.gz && \
	apt-get clean && rm -rf /var/lib/apt/lists/*

# Enable sqlx offline mode to avoid requiring database connection at compile time
ENV SQLX_OFFLINE=true

# Copy the source code
COPY . .

# Create the final bundle folder. Bundle with release build profile to enable optimizations.
RUN dx bundle --web --release --package web

FROM chef AS runtime
COPY --from=builder /app/target/dx/web/release/web/ /usr/local/app

ENV PORT=8080
ENV IP=0.0.0.0
EXPOSE 8080
WORKDIR /usr/local/app
ENTRYPOINT [ "/usr/local/app/web" ]

