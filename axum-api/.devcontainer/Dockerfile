FROM mcr.microsoft.com/devcontainers/rust:bookworm

RUN chown -R 1000:1000 /usr/local/cargo /usr/local/rustup

RUN mkdir /app && chown 1000:1000 /app

USER 1000
RUN cargo install sqlx-cli