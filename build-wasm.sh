#!/bin/bash

rm -f web/agba_wasm_bg.wasm
rm -f web/agba_wasm.js

pushd wasm
wasm-pack build --target web

mv pkg/agba_wasm_bg.wasm ../web
mv pkg/agba_wasm.js ../web
popd

