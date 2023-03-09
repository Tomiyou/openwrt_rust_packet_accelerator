#![no_std]

extern crate alloc;
use crate::alloc::string::{String, ToString};
use linux_kernel_module::c_types;
use linux_kernel_module::println;
use linux_kernel_module::bindings;
use hashbrown::HashMap;

struct Flow {
    serial: u32,
}

struct HelloWorldModule {
    flows: HashMap<u32, Flow>,
}

impl linux_kernel_module::KernelModule for HelloWorldModule {
    fn init() -> linux_kernel_module::KernelResult<Self> {
        println!("Hello from Rust!");
        Ok(HelloWorldModule {
            flows: HashMap::new(),
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
pub extern "C" fn rust_init() -> c_types::c_int {
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
pub extern "C" fn rust_cleanup() {
    unsafe {
        MODULE = None;
    }
}
