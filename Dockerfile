# Need to use Fedora, as GTK windows cross-compile packages not available on Ubuntu
FROM fedora:latest

# Install Rust via rustup.rs script, and auto agree
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

RUN dnf install -y \
    gcc \
    glib2-devel \
    gtk3-devel \
    mingw64-gcc \
    mingw64-gtk3 \
    mingw64-pango \
    mingw64-poppler \
    mingw64-winpthreads-static \
    openssl-devel

RUN source $HOME/.cargo/env && rustup target add x86_64-pc-windows-gnu

RUN source $HOME/.cargo/env && cargo install wasm-pack
