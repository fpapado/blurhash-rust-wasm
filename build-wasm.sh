#!/bin/bash

# web (bundler) build
wasm-pack build --target bundler --release --out-dir pkg

echo "[bundler target] Initial size"
wc -c pkg/blurhash_wasm_bg.wasm

echo "[bundler target] Size after gzip"
gzip -9 < pkg/blurhash_wasm_bg.wasm | wc -c

# node build
wasm-pack build --target nodejs --release --out-dir pkg-node

echo "[node target] Initial size"
wc -c pkg/blurhash_wasm_bg.wasm

echo "[node target] Size after gzip"
gzip -9 < pkg/blurhash_wasm_bg.wasm | wc -c
