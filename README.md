# rust-blurhash

A Rust implementation of the [blurhash algorithm](https://blurha.sh/).
It is also compiled to WebAssembly (WASM), and available on npm.

This crate is a work in progress!

## Usage in JS

```js
import * as blurhash from "blurhash";

// Returned as Uint8Array
const pixels = blurhash.decode("LKO2?U%2Tw=w]~RBVZRi};RPxuwH", 40, 30);

// You can use this to construct src- or canvas- compatible resources
// WIP
```

## Usage in Rust

## decode

```rust
use blurhash;

// Result<Vec<u8>, blurhash::Error>
let res = blurhash::decode("LKO2?U%2Tw=w]~RBVZRi};RPxuwH", 40, 30);
```

## encode

Not yet implemented :)

## About the setup

[**Based on the rust wasm-pack template**][template-docs]

This template is designed for compiling Rust libraries into WebAssembly and
publishing the resulting package to NPM.

Be sure to check out [other `wasm-pack` tutorials online][tutorials] for other
templates and usages of `wasm-pack`.

[tutorials]: https://rustwasm.github.io/docs/wasm-pack/tutorials/index.html
[template-docs]: https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/index.html

## 🚴 Usage

### 🛠️ Build with `wasm-pack build`

```
wasm-pack build
```

### 🔬 Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```

### 🎁 Publish to NPM with `wasm-pack publish`

```
wasm-pack publish
```

## 🔋 Batteries Included

- [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
- [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
- [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.
