#!/bin/bash

# Sets up user environment for cross compiling to Windows
# This only needs to be run once
# Assumes packages needed for Linux build are already installed (rust, SDL2, etc.)

sudo apt install -y gcc-mingw-w64-x86-64
rustup target add x86_64-pc-windows-gnu
cp -r lib/SDL2-2.0.12/x86_64-w64-mingw32/lib/* ~/.rustup/toolchains/stable-x86_64-unkown-linux-gnu/lib/rustlib/x86_64-pc-windows-gnu/lib
cp lib/SDL2-2.0.12/x86_64-w64-mingw32/bin/SDL2.dll .
