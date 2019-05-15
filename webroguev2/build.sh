rustup run nightly cargo build --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/webroguev2.wasm --out-dir .