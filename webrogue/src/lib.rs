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

#[wasm_bindgen]
pub struct Engine {
    display: Display,
    points: HashMap<GridPoint, String>,
    prize_location: Option<GridPoint>,
}

#[wasm_bindgen]
impl Engine {
    #[wasm_bindgen(constructor)]
    pub fn new(display: Display) -> Engine {
        Engine {
            display,
            points: HashMap::new(),
            prize_location: None,
        }
    }

    pub fn on_dig(&mut self, x: i32, y: i32, val: i32) {
        if val == 0 {
            let pt = GridPoint { x, y };
            self.points.insert(pt, ".".to_owned());
        }
    }

    pub fn draw_map(&self) {
        for (k,v) in &self.points {
            self.display.draw(k.x, k.y, &v);
        }
    }

    pub fn free_cell(&self, x: i32, y: i32) -> bool {
        let g = GridPoint { x, y, };
        match self.points.get(&g) {
            Some(v) => v == "." || v == "*",
            None => false,
        }
    }
}


#[derive(Serialize)]
pub struct Stats {
    pub hitpoints: i32,
    pub max_hitpoints: i32,
    pub moves: i32,
}

#[derive(PartialEq, Eq, PartialOrd, Clone, Debug, Hash)]
struct GridPoint {
    pub x: i32,
    pub y: i32,
}

#[wasm_bindgen]
pub struct PlayerCore {
    loc: GridPoint,
    moves: i32,
    display: Display,
    hp: i32,
    max_hp: i32,
    icon: String,
    color: String,
}

