#![feature(unboxed_closures)]

#[allow(dead_code)]
mod bindings {
    ::windows::include_bindings!();
}

extern crate detour;

pub mod detours;
pub mod entry;
pub mod utils;
