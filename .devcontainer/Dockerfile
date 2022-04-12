# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.202.5/containers/rust/.devcontainer/base.Dockerfile

# [Choice] Debian OS version (use bullseye on local arm64/Apple Silicon): buster, bullseye
ARG VARIANT="buster"
FROM mcr.microsoft.com/vscode/devcontainers/rust:0-${VARIANT}

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends lld python3-pip \
    && rustup update \
    && rustup toolchain install nightly --profile minimal --component clippy,rustfmt \
    && pip3 install --no-cache-dir pre-commit
COPY config /root/.cargo/config
RUN cargo install --locked bacon cargo-readme \
    && rm -rf ~/.cargo/registry
