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

fn watch(tx_wasm: Sender<RunnerCommand>) -> notify::Result<()> {
    // Creates a multi-producer (broadcaster), single-consumer (listener) channel
    // This allows cross-communications between threads.
    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher = 
        Watcher::new(tx, Duration::from_secs(1))?;
    watcher.watch(MODULE_DIR, RecursiveMode::NonRecursive)?;

    loop {
        // Block the receive channel until after a message arrives
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
                // Send message on channel, indicating that we should reload
                // the WASM module
                tx_wasm.send(RunnerCommand::Reload).unwrap();
            } else {
                println!("write (unexpected file): {:?}", path);
            }
        },
        _ => {}
    }
}


fn main() {
    let (tx_wasm, rx_wasm) = channel();
    let _indicator_runner = thread::spawn(move || {
        let mut runtime = Runtime::new();
        let mut module = wasm::get_module_instance(MODULE_FILE);
        println!("Starting wasm runner thread...");

        loop {
            // 100 milliseconds translates to 10 frames per second.
            match rx_wasm.recv_timeout(Duration_from_millis(100)) {
                Ok(RunnerCommand::Reload) => {
                    println!("Received a reload signal, sleeping for 2 secs.");
                    thread::sleep(Duration::from_secs(2));
                    module = wasm::get_module_instance(MODULE_FILE);
                },
                Ok(RunnerCommand::Stop) => {
                    runtime.shutdown();
                    break;
                },
                Err(RecvTimeoutError::Timeout) => {
                    runtime.reduce_battery();
                    runtime.advance_frame();

                    module
                        .invoke_export(
                            "sensor_update",
                            &[
                                RuntimeValue::from(wasm::SENSOR_BATTERY),
                                RuntimeValue::F64(
                                    runtime.remaining_battery.into()),
                            ][..],
                            &mut runtime,
                        ).unwrap();

                    module
                        .invoke_export(
                            "apply",
                            &[RuntimeValue::from(runtime.frame)][..],
                            &mut runtime,
                        ).unwrap();
                },
                Err(_) => break,
            }
        }
    });

    // Send channels can be cloned for multi-producer channels
    let tx_wasm_sig = tx_wasm.clone();

    // The `ctrlc` crate traps and handles SIGTERM and SIGINT signals
    ctrlc::set_handler(move || {
        tx_wasm_sig.send(RunnerCommand::Stop).unwrap();
    }).expect("Error setting Ctrl-C handler");

    // 4
    if let Err(e) = watch(tx_wasm) {
        println!("error: {:?}", e)
    }
}
