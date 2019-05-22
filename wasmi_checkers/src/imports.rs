use wasmi::{ Error as InterpreterError, FuncInstance, 
    FuncRef, ModuleImportResolver, ValueType, };

pub const PIECEMOVED_INDEX: usize = 0;
pub const PIECECROWNED_INDEX: usize = 1;

pub struct RuntimeModuleImportResolver; 
