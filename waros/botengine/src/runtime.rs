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

// Functions and indexes
const SCAN_NAME: &'static str = "scan";
const SCAN_INDEX: usize = 0;
const CANNON_NAME: &'static str = "cannon";
const CANNON_INDEX: usize = 1;
const DRIVE_NAME: &'static str = "drive";
const DRIVE_INDEX: usize = 2;
const DAMAGE_NAME: &'static str = "damage";
const DAMAGE_INDEX: usize = 3;
const SPEED_NAME: &'static str = "speed";
const SPEED_INDEX: usize = 4;
const LOCX_NAME: &'static str = "loc_x";
const LOCX_INDEX: usize = 5;
const LOCY_NAME: &'static str = "loc_y";
const LOCY_INDEX: usize = 6;
const RAND_NAME: &'static str = "rand";
const RAND_INDEX: usize = 7;
const SQRT_NAME: &'static str = "wsqrt";
const SQRT_INDEX: usize = 8;
const SIN_NAME: &'static str = "wsin";
const SIN_INDEX: usize = 9;
const COS_NAME: &'static str = "wcos";
const COS_INDEX: usize = 10;
const TAN_NAME: &'static str = "wtan";
const TAN_INDEX: usize = 11;
const ATAN_NAME: &'static str = "watan";
const ATAN_INDEX: usize = 12;
const PLOT_COURSE_NAME: &'static str = "plot_course";
const PLOT_COURSE_INDEX: usize = 13;
pub const BOTINIT_NAME: &'static str = "botinit";

/// Creates a FuncRef based on the name of the function
fn gen_funcref(name: &str) -> Option<FuncRef> {
    match name {
        SCAN_NAME => Some(FuncInstance::alloc_host(
            Signature::new(&[ValueType::I32, ValueType::I32][..], 
                           Some(ValueType::I32)),
            SCAN_INDEX,
        )),
        CANNON_NAME => Some(FuncInstance::alloc_host(
            Signature::new(&[ValueType::I32, ValueType::I32][..], 
                           Some(ValueType::I32)),
            CANNON_INDEX,
        )),
        DRIVE_NAME => Some(FuncInstance::alloc_host(
            Signature::new(&[ValueType::I32, ValueType::I32][..], 
                           Some(ValueType::I32)),
            DRIVE_INDEX,
        )),
        DAMAGE_NAME => Some(FuncInstance::alloc_host(
            Signature::new(&[][..], Some(ValueType::I32)),
            DAMAGE_INDEX,
        )),
        SPEED_NAME => Some(FuncInstance::alloc_host(
            Signature::new(&[][..], Some(ValueType::I32)),
            SPEED_INDEX,
        )),
        LOCX_NAME => Some(FuncInstance::alloc_host(
            Signature::new(&[][..], Some(ValueType::I32)),
            LOCX_INDEX,
        )),
        LOCY_NAME => Some(FuncInstance::alloc_host(
            Signature::new(&[][..], Some(ValueType::I32)),
            LOCY_INDEX,
        )),
        RAND_NAME => Some(FuncInstance::alloc_host(
            Signature::new(&[ValueType::I32][..], Some(ValueType::I32)),
            RAND_INDEX,
        )),
        SQRT_NAME => Some(FuncInstance::alloc_host(
            Signature::new(&[ValueType::I32][..], Some(ValueType::I32)),
            SQRT_INDEX,
        )),
        SIN_NAME => Some(FuncInstance::alloc_host(
            Signature::new(&[ValueType::I32][..], Some(ValueType::I32)),
            SIN_INDEX,
        )),
        COS_NAME => Some(FuncInstance::alloc_host(
            Signature::new(&[ValueType::I32][..], Some(ValueType::I32)),
            COS_INDEX,
        )),
        TAN_NAME => Some(FuncInstance::alloc_host(
            Signature::new(&[ValueType::I32][..], Some(ValueType::I32)),
            TAN_INDEX,
        )),
        ATAN_NAME => Some(FuncInstance::alloc_host(
            Signature::new(&[ValueType::I32][..], Some(ValueType::I32)),
            ATAN_INDEX,
        )),
        PLOT_COURSE_NAME => Some(FuncInstance::alloc_host(
            Signature::new(&[ValueType::I32, ValueType::I32][..],
            Some(ValueType::I32)),
            PLOT_COURSE_INDEX,
        )),
        _ => None,
    }
}