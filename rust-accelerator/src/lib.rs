#![no_std]

extern crate alloc;
use core::intrinsics::likely;

use linux_kernel_module::c_types;
use linux_kernel_module::bindings;

mod ipv4;
mod ipv6;
mod types;

static mut MODULE: Option<types::RustAccelerator> = None;

#[no_mangle]
pub extern "C" fn rust_init() -> c_types::c_int {
    match <types::RustAccelerator as linux_kernel_module::KernelModule>::init() {
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

#[no_mangle]
pub extern "C" fn rust_skb_recv(skb: *const bindings::sk_buff) -> c_types::c_int {
    let global = unsafe {
        match &MODULE {
            Some(m) => m,
            None => return 0,
        }
    };

    let protocol = unsafe {
        (*skb).__bindgen_anon_5.__bindgen_anon_1.as_ref().protocol
    };

    return ipv4::ipv4_recv(skb, global);
}
