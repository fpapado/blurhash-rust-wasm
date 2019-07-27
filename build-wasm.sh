#!/bin/bash

# The initial build
# Set wee_alloc as a smaller allocator
# We do it here instead of Cargo.toml, to keep
# the default allocator in the native Rust library build
wasm-pack build -- --features wee_alloc

echo "Initial size"

wc -c pkg/blurhash_wasm_bg.wasm

# Optimize for size
# You might need to install wasm-opt from binaryen
# https://github.com/WebAssembly/binaryen/releases
wasm-opt -Os -o pkg/blurhash_wasm_bg.wasm pkg/blurhash_wasm_bg.wasm

# echo "Size after wasm-opt"
wc -c pkg/blurhash_wasm_bg.wasm

echo "Size after gzip"
gzip -9 < pkg/blurhash_wasm_bg.wasm | wc -c