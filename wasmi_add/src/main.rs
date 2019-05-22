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
        // add.wasm was in our raw_wasm project
        let mut f = File::open("../raw_wasm/add.wasm")?;
        f.read_to_end(&mut buffer)?;
    }
    let module = wasmi::Module::from_buffer(buffer)?;

    // Create a new WASM module with default imports
    let instance = ModuleInstance::new( &module, &ImportsBuilder::default())
        .expect("Failed to instantiate WASM module...")
        .assert_no_start();

    let mut args = Vec::<RuntimeValue>::new();
    args.push(RuntimeValue::from(28));
    args.push(RuntimeValue::from(14));

    let result: Option<RuntimeValue> = 
        instance.invoke_export("add", &args, &mut NopExternals)?;

    match result {
        Some(RuntimeValue::I32(v)) => {
            println!("The answer to your question is {}", v);
        },
        Some(_) => {
            println!("What? What is that data type?!?");
        },
        None => {
            println!("Failed to get a result from the invocation!");
        },
    }
    Ok(())

}
