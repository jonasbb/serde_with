# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.202.5/containers/rust/.devcontainer/base.Dockerfile

FROM mcr.microsoft.com/devcontainers/rust:1

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends lld pipx \
    && pipx install pre-commit \
    && rustup update \
    && rustup toolchain install nightly --profile minimal --component clippy,rustfmt
COPY config /root/.cargo/config
RUN cargo install --locked bacon cargo-readme \
    && rm -rf ~/.cargo/registry
