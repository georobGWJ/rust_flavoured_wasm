use std::fmt;
use std::fs::File;
use wasmi::{
    Error as InterpreterError, Externals, FuncInstance, FuncRef,
    HostError, ImportsBuilder, Module, ModuleImportResolver, ModuleInstance,
    ModuleRef, RuntimeArgs, RuntimeValue, Signature, Trap, ValueType,
};

// 1
#[cfg(any(target_arch = "armv7", target_arch = "arm"))]
use blinkt::Blinkt;

fn load_module(path: &str) -> Module {
    use std::io::prelude::*;
    let mut file = File::open(path).unwrap();
    let mut wasm_buf = Vec::new();
    file.read_to_end(&mut wasm_buf).unwrap();
    Module::from_buffer(&wasm_buf).unwrap()
}

fn get_module_instance(path: &str) -> ModuleRef {
    let module = load_module(path);
    let mut imports = ImportsBuilder::new();
    imports.push_resolver("env", &RuntimeModuleImportResolver);

    ModuleInstance::new(&module, &imports)
        .expect("Failed to instantiate module...")
        .assert_no_start()
}

pub const SENSOR_BATTERY: i32 = 20;

#[derive(Debug)]
pub enum Error {
    Interpreter(InterpreterError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<InterpreterError> for Error {
    fn from(e: InterpreterError) -> Self {
        Error::Interpreter(e)
    }
}

impl HostError for Error {}

// 2
pub struct Runtime {
    #[cfg(any(target_arch = "armv7", target_arch = "arm"))]
    blinkt: Blinkt,
    pub frame: i32,
    pub remaining_battery: f64,
}

impl Runtime {
    #[cfg(any(target_arch = "armv7", target_arch = "arm"))]
    pub fn new() -> Runtime {
        println!("Instantiating WASM runtime (ARM)");
        Runtime {
            blinkt: Blinkt::new().unwrap(),
            frame: 0,
            remaining_battery: 100.0,
        }
    }

    #[cfg(not(any(target_arch = "armv7", target_arch = "arm")))]
    pub fn new() -> Runtime {
        println!("Instantiating WASM runtime (non-ARM)");
        Runtime {
            frame: 0,
            remaining_battery: 100.0,
        }
    }
}

impl Externals for Runtime {
    fn invoke_index(&mut self, index: usize, args: RuntimeArgs)
        -> Result<Option<RuntimeValue>, Trap> {
        match index {
            0 =>    {
                let idx: i32 = args.nth(0);
                let red: i32 = args.nth(1);
                let green: i32 = args.nth(2);
                let blue: i32 = args.nth(3);
                self.set_led(idx, red, green, blue);
                Ok(None)
            },
            _ => panic!("Unknown function index!"),
        }
    }
}