use super::imports::{ PIECECROWNED_INDEX, PIECEMOVED_INDEX };

use wasmi::{ Externals, RuntimeArgs, RuntimeValue, Trap };

// Good spot to put module-specific state
pub struct Runtime {}

impl Runtime {

    pub fn new() -> Runtime {
        Runtime {}
    }

    // Dispatcher calls this function as needed.
    fn handle_piece_moved(&self, from: (i32, i32), to: (i32, i32))
            -> Result<Option<RuntimeValue>, Trap> {
        println!("A piece was moved from ({}, {}) to ({}, {})", 
            from.0, from.1, to.0, to.1
        );
        Ok(None)
    }

    // Dispatcher calls this function as needed.
    fn handle_piece_crowned(&self, loc: (i32, i32)) -> Result<Option<RuntimeValue>, Trap> {
        println!("A piece was crowned at ({}, {})", loc.0, loc.1);
        Ok(None)
    }
}

impl Externals for Runtime {

    // Central dispatcher, converts function index to call result
    fn invoke_index(&mut self, index: usize, args: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
        match index {
            PIECECROWNED_INDEX => {
                let piece_x: i32 = args.nth(0);
                let piece_y: i32 = args.nth(1);
                self.handle_piece_crowned((piece_x, piece_y))
            },
            PIECEMOVED_INDEX => {
                let from_x: i32 = args.nth(0);
                let from_y: i32 = args.nth(1);
                let to_x: i32   = args.nth(2);
                let to_y: i32   = args.nth(3);
                self.handle_piece_moved((from_x, from_y), (to_x, to_y))
            },
            _ => panic!("unknown function index!"),
        }
    }
}