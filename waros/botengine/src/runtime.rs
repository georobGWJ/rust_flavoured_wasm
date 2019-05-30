use crate::game::{ readlock, scanner::ScannerSystem, writelock };
use crate::{ Error, Kind };
use nalgebra::Point2;
use std::sync::Arc;
use wasmi::{ 
    Error as InterpreterError, Externals, FuncInstance, FuncRef,
    ModuleImportResolver, RuntimeArgs, RuntimeValue, Signature, Trap, ValueType, 
};

/// Anchor struct for implementing the ModuleImportResolver trait
pub struct RuntimeModuleImportResolver;

/// Expose the list of host-provided functions, indexes, and signatures
/// to the WASM module(s) managed by this runtime
impl<'a> ModuleImportResolver for RuntimeModuleImportResolver {

    fn resolve_func(&self, field_name: &str, _signature: &Signature) 
        -> Result<FuncRef, InterpreterError> {
        println!("Resolving {}", field_name);
        let func_ref = gen_funcref(field_name);

        match func_ref {
            Some(fr) => Ok(fr),
            None => Err(InterpreterError::Function(field_name.to_string())),
        }
    }
}