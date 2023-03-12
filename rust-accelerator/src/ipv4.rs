extern crate alloc;
use linux_kernel_module::{c_types, println, bindings};
use core::sync::atomic::{AtomicU32, Ordering};

use crate::types::RustAccelerator;

pub struct Flow {
    /* Connection matching info */
    match_dev: usize,           /* Network device */
    protocol: u8,               /* Protocol */
    match_src_ip: u32,          /* Source IP address */
    match_dest_ip: u32,         /* Destination IP address */
    match_src_port: u16,        /* Source port */
    match_dest_port: u16,       /* Destination port */

    /* Translate info */
    xlate_src_ip: u32,                          /* Source IP address */
    xlate_dest_ip: u32,                         /* Destination IP address */
    xlate_src_port: u16,                        /* Source port */
    xlate_dest_port: u16,                       /* Destination port */

    /* Counters */
    pub rx_packet_count: AtomicU32,
	pub rx_byte_count: AtomicU32,
}

#[derive(Eq, Hash, PartialEq, Default)]
pub struct FlowKey {
    match_dev: usize,           /* Network device */
    protocol: u8,               /* Protocol */
    match_src_ip: u32,          /* Source IP address */
    match_dest_ip: u32,         /* Destination IP address */
    match_src_port: u16,        /* Source port */
    match_dest_port: u16,       /* Destination port */
}

pub extern "C" fn ipv4_recv(skb: *const bindings::sk_buff, global: &RustAccelerator) -> c_types::c_int {
    println!("Rust received network packet!");

    let flow_key = crate::ipv4::FlowKey::default();

    let flow = {
        let guard = global.ipv4_flows.lock();
    
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
