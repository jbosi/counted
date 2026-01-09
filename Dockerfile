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

# Enable sqlx offline mode to avoid requiring database connection at compile time
ENV SQLX_OFFLINE=true

# Create the final bundle folder. Bundle with release build profile to enable optimizations.
RUN dx bundle --web --release --package web

FROM chef AS runtime
COPY --from=builder /app/target/dx/web/release/web/ /usr/local/app

# set our port and make sure to listen for all connections
ENV PORT=8080
ENV IP=0.0.0.0

# expose the port 8080
EXPOSE 8080

WORKDIR /usr/local/app
ENTRYPOINT [ "/usr/local/app/web" ]

