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

// 1
fn watch(tx_wasm: Sender<RunnerCommand>) -> notify::Result<()> {
    // 1
    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher = 
        Watcher::new(tx, Duration::from_secs(1))?;
    watcher.watch(MODULE_DIR, RecursiveMode::NonRecursive)?;

    loop {
        // 2
        match rx.recv() {
            Ok(event) => handle_event(event, &tx_wasm),
            Err(e) => println!("watch error: {:?}", e),
        }
    }

}

fn handle_event(event: DebouncedEvent, tx_wasm: &Sender<RunnerCommand>) {
    // 2a
    match event {
        DebouncedEvent::NoticeWrite(path) => {
            let path = Path::new(&path);
            let filename = path.file_name().unwrap();
            if filename == "indicator.wasm" {
                tx_wasm.send(RunnerCommand::Reload).unwrap();
            } else {
                println!("write (unexpected file): {:?}", path);
            }
        },
        _ => {}
    }
}


fn main() {
    println!("Hello, world!");
}
