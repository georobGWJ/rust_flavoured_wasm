use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

// Import JS files
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Exported functions
#[wasm_bindgen]
pub fn echo_static_string() {
    alert("The sky was the color of a television, tuned to a dead channel...");
}

#[wasm_bindgen]
pub fn echo_file() {
    // hardcoded path for now
    let path = Path::new("./files/tides.txt");
    let path_desc = path.display();

    // Try to open file in Read-Only mode
    let mut file = match File::open(&path) {
        Ok(file)  => file,
        Err(e) => panic!("Couldn't open {}: {}", path_desc, e.description()),
    };

    // Now read and present the contents of the file
    let mut raw_str = String::new();
    match file.read_to_string(&mut raw_str) {
        Ok(_)  => {
            let contents: &str = &raw_str[..]; // Take a fullslice of the String
            alert(contents);
        },
        Err(e) => panic!("Couldn't read {}: {}", path_desc, e.description()),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
