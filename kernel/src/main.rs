#![no_std]
#![no_main]
#![feature(core_intrinsics, const_black_box, panic_info_message, let_chains)]

extern crate libadamant;

pub mod hw;
pub mod panic;
pub mod pmm;