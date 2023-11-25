#!/bin/bash

# The initial build
wasm-pack build

echo "Initial size"

wc -c pkg/blurhash_wasm_bg.wasm

# Optimize wasm
# NOTE: Again, setting -Os did not decrease the size,
# so might as well optimise for speed.
# You might need to install wasm-opt from binaryen:
# https://github.com/WebAssembly/binaryen/releases
wasm-opt pkg/blurhash_wasm_bg.wasm -O3 -o pkg/blurhash_wasm_bg.wasm

echo "Size after wasm-opt"
wc -c pkg/blurhash_wasm_bg.wasm

echo "Size after gzip"
gzip -9 < pkg/blurhash_wasm_bg.wasm | wc -c
