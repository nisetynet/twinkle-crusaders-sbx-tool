#![feature(once_cell)]
#![feature(naked_functions)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![feature(link_llvm_intrinsics)]
pub mod fileapi;
pub mod libloaderapi;
pub mod memoryapi;
pub mod processthreadsapi;
pub mod synchapi;
pub mod utility;
pub mod winuser;
