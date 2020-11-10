FROM rust:latest

RUN apt-get update && apt-get install -y \
    gcc-arm-linux-gnueabihf \
    g++-arm-linux-gnueabihf \
    cmake \
    mingw-w64 \
    zip

# Add Windows PC and ARM Linux targets
RUN rustup target add x86_64-pc-windows-gnu
RUN rustup target add arm-unknown-linux-gnueabihf

# Install wasm-pack
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
