# blurhash-wasm

A Rust implementation of the [blurhash algorithm](https://github.com/woltapp/blurhash).

It is compiled to WebAssembly (WASM), and [available on npm as `blurhash-wasm`](https://npmjs.com/blurhash-wasm).

BlurHash is an algorithm written by [Dag Ågren](https://github.com/DagAgren) and folks at [Wolt (woltapp/blurhash)](https://github.com/woltapp/blurhash). BlurHash is "a compact representation of a placeholder for an image." It enables you to store that representation in your database. It can then be transferred together with the initial data, in order to decode and show it, before the main image request has finished (or even started).

[Online Demo](https://blurhash-wasm.netlify.com).

## Usage in JS

### Installation

You will need a package manager, either npm ([comes with node](https://nodejs.org/en/download/)) or [yarn](https://yarnpkg.com/lang/en/docs/install/).

You will also need a bundler, [webpack](https://webpack.js.org/) or [Rollup](https://rollupjs.org/guide/en/), configured for your project.

Then, in a terminal:

```shell
npm install blurhash-wasm
# Or, yarn add blurhash-wasm
```

The [demo app source](/demo) has a complete example of using `blurhash-wasm`.

### decode

```js
import * as blurhash from "blurhash-wasm";

// Returned as Uint8Array | undefined
// You can use this to construct canvas-compatible resources
const pixels = blurhash.decode("LKO2?U%2Tw=w]~RBVZRi};RPxuwH", 40, 30);
```

### encode

Implented, aim to be published in 0.3.0

## Usage in Rust

### Installation

You will need [Rust and Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html).

Add the version you want to `Cargo.toml`:

```
[dependencies]
blurhash-wasm = "0.1.0"
```

### decode

```rust
use blurhash_wasm;

// Result<Vec<u8>, blurhash::Error>
let res = blurhash::decode("LKO2?U%2Tw=w]~RBVZRi};RPxuwH", 40, 30);
```

### encode

Implented, aim to be published in 0.3.0

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
