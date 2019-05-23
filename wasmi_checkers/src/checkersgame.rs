use super::imports::RuntimeModuleImportResolver;
use super::runtime::Runtime;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use wasmi::{ ExternVal, ImportsBuilder, MemoryRef, Module, ModuleImportResolver,
             ModuleInstance, ModuleRef, RuntimeValue };

type Result<T> = ::std::result::Result<T, Box<Error>>;
type Coordinate = (i32, i32);

#[derive(Debug)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Debug)]
pub struct CheckersGame {
    runtime: Runtime,
    module_instance: ModuleRef,
}

impl CheckersGame {

    pub fn new(module_file: &str) -> CheckersGame {
        let resolver = RuntimeModuleImportResolver::new();

        let instance = load_instance(&resolver, module_file).unwrap();
        let runtime = Runtime::new();

        CheckersGame {
            module_instance: instance,
            runtime,
        }
    }

    // 1
    pub fn init(&mut self) -> Result<()> {
        self.module_instance
            .invoke_export("initBoard", &[], &mut self.runtime)?;
        OK(())
    }

    // 2
    pub fn move_piece(&mut self, from: &Coordinate, to: &Coordinate) -> Result<bool> {
        let res = self.module_instance.invoke_export( "move",
            &[
                RuntimeValue::from(from.0),
                RuntimeValue::from(from.1),
                RuntimeValue::from(to.0),
                RuntimeValue::from(to.1),
            ],
            &mut self.runtime,
        )?;

        match res {
            Some(RuntimeValue::I32(v)) => Ok(v != 0),
            _ => {
                println!("Did not get an appropriate response from the move.");
                Ok(false)
            }
        }
    }

    // 3
    pub fn get_turn_owner(&mut self) -> Result<PieceColor> {
        let res = self
            .module_instance
            .invoke_export("getTurnOwner", &[], &mut self.runtime)?;

        match res {
            Some(RuntimeValue::I32(v)) => {
                if v == 1 { Ok(PieceColor::Black) } else
                          { Ok(PieceColor::White) } 
            },
            _ => Err(From::from("Bad invocation.")),
        }
    }
}



// This function takes any struct implementing the ModuleImportResolver trait.
// That's set by '&impl ModuleImportResolver'
fn load_instance(import_resolver: &impl ModuleImportResolver, module_file: &str)
                -> Result<ModuleRef> {
    let mut buffer = Vec::new();
    let mut f = File::open(module_file)?;
    f.read_to_end(&mut buffer)?;
    let module = Module::from_buffer(buffer)?;

    let mut builder = ImportsBuilder::new();
    builder.push_resolver("events", import_resolver);

    Ok(ModuleInstance::new(&module, &builder)
        .expect("Failed to instantiate WASM module...")
        .assert_no_start())
}