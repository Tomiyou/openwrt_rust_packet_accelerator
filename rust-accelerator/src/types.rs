extern crate alloc;
use alloc::sync::Arc;
use linux_kernel_module::println;
use linux_kernel_module::sync::Spinlock;
use hashbrown::HashMap;
use hash32::{BuildHasherDefault, FnvHasher};

use crate::ipv4;
use crate::ipv6;

type FlowKeyHasher = BuildHasherDefault<FnvHasher>;

pub struct RustAccelerator {
    pub ipv4_flows: Spinlock<HashMap<ipv4::FlowKey, Arc<ipv4::Flow>, FlowKeyHasher>>,
    pub ipv6_flows: Spinlock<HashMap<ipv6::FlowKey, Arc<ipv6::Flow>, FlowKeyHasher>>,
}

impl linux_kernel_module::KernelModule for RustAccelerator {
    fn init() -> linux_kernel_module::KernelResult<Self> {
        println!("Hello from Rust!");

        Ok(RustAccelerator {
            ipv4_flows: Spinlock::new(HashMap::default()),
            ipv6_flows: Spinlock::new(HashMap::default()),
        })
    }
}

impl Drop for RustAccelerator {
    fn drop(&mut self) {
        println!("Goodbye from Rust!");
    }
}

pub enum AccelError {
    UnsupportedProtocol,
    WrongProtocol,
    ParsingError,
    FlowNotFound,
}
