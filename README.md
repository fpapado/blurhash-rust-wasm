# rust-blurhash

A Rust implementation of the [blurhash algorithm](https://blurha.sh/).

This crate is a work in progress!

## Usage

## decode

```rust
use blurhash;

// Result<Vec<u8>, blurhash::Error>
let res = blurhash::decode("LKO2?U%2Tw=w]~RBVZRi};RPxuwH", 40, 30);
```

## encode

Not yet implemented :)
