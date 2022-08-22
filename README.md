# Combinatorica

A game made in Rust for the second Bevy Jam.

## Prerequisites

```bash
rustup target install wasm32-unknown-unknown +nightly
cargo install wasm-server-runner
cargo install wasm-bindgen-cli
```

## Run locally

```bash
cargo run
```

## Deploy locally
Does not work for me on Firefox for some reason, but works on Chrome.

### Build
```bash
cargo build --release
wasm-bindgen --out-dir out/pkg --target web target/wasm32-unknown-unknown/release/combinatorica.wasm
rsync -a assets/ out/assets/
```

### Deploy
Build, then run this:
```bash
cd out
python3 -m http.server <port>
```
