// A Pattern for creating a Rust wasm-host
//  1)  Create an imports resolver that provides a signature and invocation index for
//      each imported function.    See imports.rs
//  2)  Create a Runtime for Externals.    See runtime.rs
//  3)  Create API Wrapper for exported functions

fn main() {
    println!("Hello, world!");
}
