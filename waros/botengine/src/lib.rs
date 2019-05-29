// This is the root of the engine and contains the Combatant struct
// and behavior. The combatant is a wrapper atound the loading, parsing,
// and interpreting of the WASM modules.

use std::format;
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

        thread::spawn(move || {
            let module = Module::from_buffer(&buffer).unwrap();
            // Creates a new Runtime to host the WASM module and
            // passes it a reference to the game state.
            let mut runtime = runtime::Runtime::init(game_state, n.clone());
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
