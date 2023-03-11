#![no_std]

extern crate alloc;
use crate::alloc::string::{String, ToString};
use alloc::sync::Arc;
use linux_kernel_module::c_types;
use linux_kernel_module::println;
use linux_kernel_module::bindings;
use linux_kernel_module::sync::{Mutex, MutexGuard};
use hashbrown::HashMap;
use core::sync::atomic::Ordering;

mod ipv4;

struct HelloWorldModule {
    ipv4_flows: Mutex<HashMap<ipv4::FlowKey, Arc<ipv4::Flow>>>,
}

impl linux_kernel_module::KernelModule for HelloWorldModule {
    fn init() -> linux_kernel_module::KernelResult<Self> {
        println!("Hello from Rust!");
        Ok(HelloWorldModule {
            ipv4_flows: Mutex::new(HashMap::new()),
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

#[no_mangle]
pub extern "C" fn rust_skb_recv(skb: *const bindings::sk_buff) -> c_types::c_int {
    println!("Rust received network packet!");

    let flow_key = crate::ipv4::FlowKey::default();

    let flow = {
        let module = unsafe {
            match &MODULE {
                Some(m) => m,
                None => return 0,
            }
        };

        let guard = module.ipv4_flows.lock();
    
        let flow = match guard.get(&flow_key) {
            Some(flow) => flow,
            None => return 0,
        };

        flow.clone()
    };

    println!("Found SKB flow !!!");
    flow.rx_packet_count.fetch_add(1, Ordering::AcqRel);
    flow.rx_byte_count.fetch_add(1, Ordering::AcqRel);

    return 0;
}
