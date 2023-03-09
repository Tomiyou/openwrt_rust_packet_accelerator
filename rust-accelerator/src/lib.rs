#![no_std]

extern crate alloc;
use crate::alloc::string::{String, ToString};
use linux_kernel_module::c_types;
use linux_kernel_module::println;
use linux_kernel_module::bindings;

struct HelloWorldModule {
    message: String,
}

impl linux_kernel_module::KernelModule for HelloWorldModule {
    fn init() -> linux_kernel_module::KernelResult<Self> {
        println!("Hello from Rust!");
        Ok(HelloWorldModule {
            message: "Hello World!".to_string(),
        })
    }
}

impl Drop for HelloWorldModule {
    fn drop(&mut self) {
        println!("Goodbye from Rust!");
    }
}

static mut MODULE: Option<HelloWorldModule> = None;

#[no_mangle]
pub extern "C" fn init_module() -> c_types::c_int {
    match <HelloWorldModule as linux_kernel_module::KernelModule>::init() {
        Ok(m) => {
            unsafe {
                MODULE = Some(m);
            }
            return 0;
        }
        Err(_e) => {
            return 1;
        }
    }
}

#[no_mangle]
pub extern "C" fn cleanup_module() {
    unsafe {
        MODULE = None;
    }
}

#[no_mangle]
#[link_section = ".modinfo"]
pub static LICENSE: [u8; 12] = *b"license=GPL\0";

#[no_mangle]
#[link_section = ".modinfo"]
pub static AUTHOR: [u8; 10] = *b"author=ME\0";

#[no_mangle]
#[link_section = ".modinfo"]
pub static DESCRIPTION: [u8; 29] = *b"description=My kernel module\0";
