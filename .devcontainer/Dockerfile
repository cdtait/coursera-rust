FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye

# Install additional packages and tools needed for Rust development and CI
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends \
        cmake \
        libssl-dev \
        pkg-config \
        git \
        build-essential \
        ca-certificates \
        curl \
        gnupg \
        lsb-release \
    && apt-get autoremove -y && apt-get clean -y && rm -rf /var/lib/apt/lists/*

# Switch to vscode user for Rust toolchain setup
USER vscode

# Ensure we have the latest stable Rust with required components
RUN rustup update stable \
    && rustup default stable \
    && rustup component add rustfmt clippy \
    && cargo install cargo-watch \
    && rustc --version && cargo --version

# Set environment variables for better Rust development experience
ENV CARGO_TERM_COLOR=always
ENV RUST_BACKTRACE=1

# Set the default command
CMD ["/bin/bash"]