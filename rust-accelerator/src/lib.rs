#![no_std]

extern crate alloc;

use linux_kernel_module::c_types;
use linux_kernel_module::bindings;
use types::RustAccelerator;

mod ipv4;
mod ipv6;
mod types;

static mut MODULE: Option<RustAccelerator> = None;

#[no_mangle]
pub extern "C" fn rust_init() -> c_types::c_int {
    match <RustAccelerator as linux_kernel_module::KernelModule>::init() {
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

fn get_global_data() -> &'static RustAccelerator {
    unsafe {
        match &MODULE {
            Some(m) => m,
            None => panic!("What the fuck")
        }
    }
}
