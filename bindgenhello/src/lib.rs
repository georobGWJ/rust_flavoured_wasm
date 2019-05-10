// Build using:
//    rustup run nightly cargo build --target wasm32-unknown-unknown
// then
//    wasm-bindgen target/wasm32-unknown-unknown/debug/bindgenhello.wasm --out-dir .

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

// Decorating Rust code with #[wasm_bindgen] will cause the compiler to both 
// generate code for the wasm file as well as metadata to help generate
// associated JavaScript output created by the wasm-bindgen cli tools

// Import 'window.alert'
// Bind this alert function to the alert() JavaScript function
// With this, any Rust code that invokes alert will be converted into code that
// invokes JS alert()
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// Export a 'hello' function that accepts a reference to a string
// Pure wasm cannot handle strings. Tyis created the needed conversions
// to allow complex data structures to go back and forth between wasm and JS
#[wasm_bindgen]
pub fn hello(name: &str) {
    alert(&format!("Hello, {}!", name));
}