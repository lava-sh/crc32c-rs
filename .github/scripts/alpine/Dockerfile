FROM docker.io/library/python:3.14-alpine

RUN apk add --no-cache build-base

COPY --from=rustlang/rust:nightly-alpine /usr/local/cargo /usr/local/cargo
COPY --from=rustlang/rust:nightly-alpine /usr/local/rustup /usr/local/rustup

ENV PATH="/usr/local/cargo/bin:${PATH}" RUSTUP_HOME=/usr/local/rustup CARGO_HOME=/usr/local/cargo

COPY --from=ghcr.io/astral-sh/uv:latest /uv /bin/
COPY entrypoint.sh /entrypoint.sh

RUN chmod +x /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]