FROM rust:1 AS chef
RUN cargo install cargo-chef

# Install `dx`
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall dioxus-cli --root /.cargo -y --force
ENV PATH="/.cargo/bin:$PATH"

# Copy migration folder
COPY --from=builder /app/migrations /usr/local/app/migrations

# install runtime system packages and sqlx-cli
RUN apt-get update && apt-get install -y --no-install-recommends \
	postgresql-client && \
	rm -rf /var/lib/apt/lists/*
RUN cargo install sqlx-cli --locked --root /usr/local/cargo && \
	ln -s /usr/local/cargo/bin/sqlx /usr/local/bin/sqlx

ENV PATH="/usr/local/cargo/bin:${PATH}"