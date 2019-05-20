// Compile using:
// cargo web build --target=wasm32-unknown-unknown

// Start a Dev server using:
// cargo web start --target=wasm32-unknown-unknown


extern crate yew;
extern crate yew_counter;  // Access to ./src/lib.rs

use yew::prelude::*;
use yew::services::console::ConsoleService;
use yew_counter::Model;

pub struct Context {
    console: ConsoleService,
}

impl AsMut<ConsoleService> for Context {
    fn as_mut(&mut self) -> &mut ConsoleService {
        &mut self.console
    }
}

fn main() {
    yew::initialize();
    let context = Context {
        console: ConsoleService::new(),
    };

    let app: App<_, Model> = App::new(context);
    app.mount_to_body();
    yew::run_loop();
}
