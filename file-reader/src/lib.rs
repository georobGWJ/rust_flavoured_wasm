extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

// Import JS files
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
