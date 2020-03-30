#!/bin/bash

DEBUG=false
BUILD_FLAGS=""

while test $# -gt 0; do
    case "$1" in
        -d|--debug)
            DEBUG=true
            BUILD_FLAGS="$BUILD_FLAGS --debug"
            shift
            ;;
        *)
            break
            ;;
    esac
done

rm -f web/agba_wasm.wasm

pushd wasm
wasm-pack build --target web $BUILD_FLAGS
popd

if $DEBUG; then
    mv target/wasm32-unknown-unknown/debug/agba_wasm.wasm web
else
    mv target/wasm32-unknown-unknown/release/agba_wasm.wasm web
fi
