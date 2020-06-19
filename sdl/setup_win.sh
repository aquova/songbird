#!/bin/bash

# Sets up user environment for cross compiling to Windows
# This only needs to be run once
# Assumes packages needed for Linux build are already installed (rust, SDL2, etc.)

sudo apt install -y gcc-mingw-w64-x86-64
rustup target add x86_64-pc-windows-gnu
