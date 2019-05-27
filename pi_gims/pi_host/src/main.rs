#[cfg(any(target_arch = "armv7", target_arch = "arm"))]
extern crate blinkt;

extern crate ctrlc;
extern crate notify;
extern crate wasmi;

use notify::{ DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher };
use std::path::Path;
use std::sync::mpsc::{ channel, RecvTimeoutError, Sender };
use std::thread;
use std::time::Duration;
use wasm::Runtime;
use wasmi::RuntimeValue;

const MODULE_FILE: &'static str = 
    "/Users/georob/Desktop/spike/wasm/rust_flavoured_wasm/pi_gims/indicator.wasm";
const MODULE_DIR: &'static str = 
    "/Users/georob/Desktop/spike/wasm/rust_flavoured_wasm/pi_gims/";

enum RunnerCommand {
    Reload,
    Stop,
}

fn main() {
    println!("Hello, world!");
}
