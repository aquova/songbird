FROM rust:latest

RUN apt-get update && apt-get install -y \
    cmake \
    mingw-w64 \
    zip

RUN rustup target add x86_64-pc-windows-gnu

# Install wasm-pack
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
