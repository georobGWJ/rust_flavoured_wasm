extern crate stdweb;
#[macro_use]
extern crate yew;

use stdweb::web::Date;
use yew::prelude::*;
use yew::services::console::ConsoleService;

// Models the value of the Counter
pub struct Model {
    value: i64,
}

// We pass Messages through the Component to change the model and thus the rendering
pub enum Msg {
    Increment,
    Decrement,
    Bulk(Vec<Msg>),
}

// This indicates that for any instantiated Model struct, the scope contains a
// bound Component trait implementation when the type parameter to generic <C>
// implements the AsMut<ConsoleService> trait.
impl<C> Component<C> for Model
    where C: AsMut<ConsoleService> {
        type Message = Msg;
        type Properties = ();

        fn create(_: Self::Properties, _: &mut Env<C, Self>) -> Self {
            Model { value: 0, }
        }

        fn update(&mut self, msg: Self::Message, 
                  env: &mut Env<C, Self>) -> ShouldRender {
            match msg {
                Msg::Increment => {
                    self.value = self.value + 1;
                    env.as_mut().log("plus one");
                },
                Msg::Decrement => {
                    self.value = self.value - 1;
                    env.as_mut().log("minus one");
                },
                Msg::Bulk(list) => for msg in list {
                    self.update(msg, env);
                    env.as_mut().log("Bulk action");
                },
            }
            // Should this render? For this example, yes. Always.
            true
        }
    }

// The Renderable trait allows the Model struct to render anything that is
// of type Model with a context type parameter that allows us to obtain a mutable
// reference to a ConsoleService
// 'static is a lifetime specifier
impl<C> Renderable<C, Model> for Model
    where C: AsMut<ConsoleService> + 'static {
        
}