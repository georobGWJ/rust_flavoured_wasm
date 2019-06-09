// This is the root of the engine and contains the Combatant struct
// and behavior. The combatant is a wrapper atound the loading, parsing,
// and interpreting of the WASM modules.

extern crate wasmi;
extern crate nalgebra;
extern crate rand;

use std::fmt;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use wasmi:: { HostError, ImportsBuilder, Module, ModuleInstance, ModuleRef };

pub use crate::game::{ GameState, Gameloop };
pub use crate::runtime::{ Runtime, BOTINIT_NAME };

pub struct Combatant {}

impl Combatant {
    pub fn buffer_from_file(path: &str) -> Result<Vec<u8>> {
        use std::fs::File;
        use std::io::prelude::*;

        let mut file = File::open(path)?;
        let mut wasm_buf = Vec::new();
        let _bytes_read = file.read_to_end(&mut wasm_buf)?;
        Ok(wasm_buf)
    }

    pub fn start(name: &str, 
                 buffer: Vec<u8>, 
                 // Arc (Atomically Reference Counted) lets us share 
                 // pointers to the GameState struct.
                 game_state: Arc<crate::game::GameState>)
        // This would return a JoinHandle, except this is an infinite
        // loop, so it will probably never return from here.
        -> JoinHandle<()> {
        let n = name.to_string();

        // Each Combatant (bot) runs in it's own thread (verify)
        thread::spawn(move || {
            let module = Module::from_buffer(&buffer).unwrap();
            // Creates a new Runtime to host the WASM module and
            // passes it a reference to the game state.
            let mut runtime = runtime::Runtime::init(game_state, n.clone());

            let moduleref = Self::get_module_instance_from_module(&module).unwrap();
            // Invokes the bot_init() function in the WASM module, starting
            // the robot's infinite loop.
            let res = moduleref.invoke_export(BOTINIT_NAME, &[][..], &mut runtime);

            println!("bot init loop exited for player {} - {:?}", n, res);
        })
    }

    fn get_module_instance_from_module(module: &Module) ->  Result<ModuleRef> {
        let mut imports = ImportsBuilder::new();
        imports.push_resolver("env", &runtime::RuntimeModuleImportResolver);
        
        Ok(ModuleInstance::new(module, &imports)
            .expect("Failed to instantiate module")
            .assert_no_start())
    }
}

/// A botengine error
#[derive(Debug)]
pub struct Error {
    kind: Kind,
}

/// Implements the wasmi HostError trait
impl HostError for Error {}

/// Implement standard error trait for the botengine error
impl std::error::Error for Error {

    fn description(&self) -> &str {
        "A botengine library error occurred."
    }
    
    fn cause(&self) -> Option<&std::error::Error> {
        None
    }
}

/// Ensure that the botengine error can be string formatted
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            Kind::InterpreterError(ref s) => fmt::Display::fmt(s, f),
            Kind::MiscFailure(ref s) => fmt::Display::fmt(s, f),
            Kind::IoError(ref s) => fmt::Display::fmt(s, f),
            Kind::ExportResolve(ref s) => fmt::Display::fmt(s, f),
        }
    }
}

/// Creates a botengine error from an I/O Error
impl From<std::io::Error> for Error {
    fn from(source: std::io::Error) -> Error {
        Error {
            kind: Kind::IoError(source),
        }
    }
}

/// Creates a botengine error from an Interpreter Error
impl From<wasmi::Error> for Error {
    fn from(source: wasmi::Error) -> Error {
        Error {
            kind: Kind::InterpreterError(source),
        }
    }
}

/// Indicates the kind of error that occurred
#[derive(Debug)]
enum Kind {
    InterpreterError(wasmi::Error),
    IoError(std::io::Error),
    ExportResolve(String),
    MiscFailure(String),
}

/// A Result where failure is a botengine error
pub type Result<T> = std::result::Result<T, Error>;

mod events;
mod game;
mod runtime;

