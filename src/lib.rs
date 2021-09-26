#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

mod messageram;
pub mod pac;
use pac::RegisterBlock;
use pac::generic::*;

pub unsafe trait Instance {
    /// Pointer to the instance's register block.
    const REGISTERS: *mut RegisterBlock;
}