FROM rust:1.85 AS chef

# Use cache mounts for cargo installations to speed up rebuilds
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    cargo install cargo-chef

# Install `dx`
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    cargo binstall dioxus-cli --root /.cargo -y --force

# install runtime system packages and sqlx-cli
RUN apt-get update && apt-get install -y --no-install-recommends \
	postgresql-client && \
	rm -rf /var/lib/apt/lists/*
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    cargo install sqlx-cli --locked --root /usr/local/cargo && \
	ln -s /usr/local/cargo/bin/sqlx /usr/local/bin/sqlx

ENV PATH="/.cargo/bin:$PATH"