use wasmi::{ Error as InterpreterError, FuncInstance, 
    FuncRef, ModuleImportResolver, ValueType, };

pub const PIECEMOVED_INDEX: usize = 0;
pub const PIECECROWNED_INDEX: usize = 1;

pub struct RuntimeModuleImportResolver;

impl RuntimeModuleImportResolver {

    pub fn new() -> RuntimeModuleImportResolver {
        RuntimeModuleImportResolver {}
    }

}

impl <'a> ModuleImportResolver for RuntimeModuleImportResolver {

    fn resolve_func(&self, field_name: &str, _signature: &Signature)
                    -> Result<FuncRef, InterpreterError> {

        let func_ref = match field_name {
            // Provide a FuncRef for the piecemoved() function
            "piecemoved" => FuncInstance::alloc_host(
                Signature::new(
                    &[ 
                        ValueType: I32, ValueType: I32, 
                        ValueType: I32, ValueType: I32 
                    ][..],
                    None,
                ),
                PIECEMOVED_INDEX,
            ),
            // Provide a FuncRef for the piececrowned() function
            "piececrowned" => FuncInstance::alloc_host(
                Signature::new(
                    &[ValueType: I32, ValueType: I32,][..], None),
                PIECECROWNED_INDEX,
            ),
            // Return an Error when trying to import an unknown function
            _ => {
                return Err(InterpreterError::Function(format!(
                    "host module does not export function: {}", field_name
                )))
            }
        };
        Ok(func_ref)
    }

}
