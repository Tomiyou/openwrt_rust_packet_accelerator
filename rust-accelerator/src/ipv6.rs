extern crate alloc;
use linux_kernel_module::{c_types, println, bindings};
use core::sync::atomic::{AtomicU32, Ordering};

pub type Ipv6Addr = u128;

pub struct Flow {
    /* Connection matching info */
    match_dev: usize,           /* Network device */
    protocol: u8,               /* Protocol */
    match_src_ip: Ipv6Addr,     /* Source IP address */
    match_dest_ip: Ipv6Addr,    /* Destination IP address */
    match_src_port: u16,        /* Source port */
    match_dest_port: u16,       /* Destination port */

    /* Translate info */
    xlate_src_ip: Ipv6Addr,     /* Source IP address */
    xlate_dest_ip: Ipv6Addr,    /* Destination IP address */
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
    match_src_ip: Ipv6Addr,     /* Source IP address */
    match_dest_ip: Ipv6Addr,    /* Destination IP address */
    match_src_port: u16,        /* Source port */
    match_dest_port: u16,       /* Destination port */
}

#[no_mangle]
pub fn rust_accel_recv_ipv6(dev: *const bindings::net_device, skb: *const bindings::sk_buff, pkt_len: u32) -> c_types::c_int {
    /* First parse the packet headers */
    println!("Rust received network packet!");

    let packet_headers = unsafe {
        core::slice::from_raw_parts((*skb).data, pkt_len as usize)
    };

    match pdu::Ip::new(packet_headers) {
        Ok(pdu::Ip::Ipv6(ipv6_pdu)) => {
            println!("[ipv6] source_address: {:x?}", ipv6_pdu.source_address().as_ref());
            println!("[ipv6] destination_address: {:x?}", ipv6_pdu.destination_address().as_ref());
            println!("[ipv6] protocol: 0x{:02x}", ipv6_pdu.computed_protocol());
            // upper-layer protocols can be accessed via the inner() method (not shown)
        }
        Ok(pdu::Ip::Ipv4(ipv4_pdu)) => {
            println!("Wrong IPv4 packet {:?}", ipv4_pdu);
            return 0;
        }
        Err(e) => {
            println!("Unknown packet protocol {:?}", e);
            return 0;
        }
    }

    let global = crate::get_global_data();

    let flow_key = FlowKey::default();

    let flow = {
        let guard = global.ipv6_flows.lock();

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
