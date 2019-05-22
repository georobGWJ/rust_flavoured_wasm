extern crate wasmi;

use std::result::Result;
use std::boxed::Box;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use wasmi::{ ImportsBuilder, ModuleInstance, NopExternals, RuntimeValue };

// main() here will return a Generic Result
fn main() -> Result<(), Box<Error>>{
    // Read in the wasm bytes
    let mut buffer = Vec::new();
    {
        let mut f = File::open("../fundamentals/add.wasm")?; // Sloppy error handling
        f.read_to_end(&mut buffer)?;
    }
    let module = wasmi::Module::from_buffer(buffer)?;
}
