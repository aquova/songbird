#!/bin/bash

DEBUG=false

while test $# -gt 0; do
    case "$1" in
        -d|--debug)
            DEBUG=true
            shift
            ;;
        *)
            break
            ;;
    esac
done

rm -f web/agba_wasm.wasm
pushd wasm
if $DEBUG; then
    wasm-pack build --target web --debug
    mv target/wasm32-unknown-unknown/debug/agba_wasm.wasm ../web
else
    wasm-pack build --target web
    mv target/wasm32-unknown-unknown/release/agba_wasm.wasm ../web
fi
popd
