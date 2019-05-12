#[macro_use]
extern crate serde_derive;

extern crate wasm_bindgen;

use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // For invoking JS alert()
    fn alert(s: &str);

    // Import JS console.log() from console
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // Import JS stats_updated() from 'index' module 
    #[wasm_bindgen(module = "./index")]
    fn stats_updated(stats: JsValue);

    pub type Display;

    // Make ROT.js library methods available
    #[wasm_bindgen(method, structural, js_namespace = ROT)]
    fn draw(this: &Display, x: i32, y: i32, ch: &str);

    #[wasm_bindgen(method, structural, js_name = draw, js_namespace = ROT)]
    fn draw_color(this: &Display, x: i32, y: i32, ch: &str, color: &str);

}