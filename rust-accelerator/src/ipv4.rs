extern crate alloc;
use linux_kernel_module::{c_types, println, bindings};
use core::sync::atomic::{AtomicU32, Ordering};

pub type Ipv4Addr = u32;

pub struct Flow {
    /* Connection matching info */
    match_dev: usize,           /* Network device */
    protocol: u8,               /* Protocol */
    match_src_ip: Ipv4Addr,     /* Source IP address */
    match_dest_ip: Ipv4Addr,    /* Destination IP address */
    match_src_port: u16,        /* Source port */
    match_dest_port: u16,       /* Destination port */

    /* Translate info */
    xlate_src_ip: Ipv4Addr,     /* Source IP address */
    xlate_dest_ip: Ipv4Addr,    /* Destination IP address */
    xlate_src_port: u16,        /* Source port */
    xlate_dest_port: u16,       /* Destination port */

    /* Counters */
    pub rx_packet_count: AtomicU32,
	pub rx_byte_count: AtomicU32,
}

#[derive(Eq, Hash, PartialEq, Default)]
pub struct FlowKey {
    match_dev: usize,           /* Network device */
    protocol: u8,               /* Protocol */
    match_src_ip: Ipv4Addr,     /* Source IP address */
    match_dest_ip: Ipv4Addr,    /* Destination IP address */
    match_src_port: u16,        /* Source port */
    match_dest_port: u16,       /* Destination port */
}

#[no_mangle]
pub fn rust_accel_recv_ipv4(dev: *const bindings::net_device, skb: *const bindings::sk_buff) -> c_types::c_int {
    println!("Rust received network packet!");

    let global = crate::get_global_data();

    let flow_key = FlowKey::default();

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
