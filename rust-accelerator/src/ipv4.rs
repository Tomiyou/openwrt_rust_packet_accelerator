#![no_std]

extern crate alloc;
use linux_kernel_module::c_types;
use linux_kernel_module::println;
use linux_kernel_module::bindings;
use hashbrown::HashMap;
use core::sync::atomic::AtomicU32;

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
