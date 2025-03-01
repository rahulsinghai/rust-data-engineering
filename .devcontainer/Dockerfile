FROM mcr.microsoft.com/devcontainers/rust:latest

# Include lld linker to improve build times either by using environment variable
# RUSTFLAGS="-C link-arg=-fuse-ld=lld" or with Cargo's configuration file (i.e see .cargo/config.toml).
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
   && apt-get -y install clang lld \
   && apt-get autoremove -y && apt-get clean -y
RUN rustup toolchain install stable
RUN rustup update stable
# Change default to latest stable
RUN rustup default stable-x86_64-unknown-linux-gnu