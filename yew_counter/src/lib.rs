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

    }

// The Renderable trait allows the Model struct to render anything that is
// of type Model with a context type parameter that allows us to obtain a mutable
// reference to a ConsoleService
// 'static is a lifetime specifier
impl<C> Renderable<C, Model> for Model
    where C: AsMut<ConsoleService> + 'static {
        
}