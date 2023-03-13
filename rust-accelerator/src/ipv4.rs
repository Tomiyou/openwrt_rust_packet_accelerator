extern crate alloc;
use alloc::sync::Arc;
use linux_kernel_module::{c_types, println, bindings};
use core::sync::atomic::{AtomicU32, Ordering};

use crate::types::AccelError;

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
    dev: usize,           /* Network device */
    protocol: u8,         /* Protocol */
    src_ip: Ipv4Addr,     /* Source IP address */
    dest_ip: Ipv4Addr,    /* Destination IP address */
    src_port: u16,        /* Source port */
    dest_port: u16,       /* Destination port */
}

fn do_flow_lookup(flow_key: &FlowKey) -> Result<Arc<Flow>, AccelError> {
    let global = crate::get_global_data();

    let guard = global.ipv4_flows.lock();

    let flow = match guard.get(flow_key) {
        Some(flow) => flow,
        None => {
            println!("Flow not found!");
            return Err(AccelError::FlowNotFound);
        }
    };

    Ok(flow.clone())
}

#[inline]
fn accel_udp(skb: *const bindings::sk_buff, ip_header: &pdu::Ipv4Pdu, udp_header: &pdu::UdpPdu) -> Result<(), AccelError> {
    let protocol = ip_header.protocol();
    let src_ip = u32::from_be_bytes(ip_header.source_address());
    let dest_ip = u32::from_be_bytes(ip_header.destination_address());
    let src_port = udp_header.source_port();
    let dest_port = udp_header.destination_port();

    let flow_key = FlowKey {
        dev: 0,
        protocol,
        src_ip,
        dest_ip,
        src_port,
        dest_port
    };

    /* Lookup flow using key */
    let flow = do_flow_lookup(&flow_key)?;

    println!("Found SKB flow !!!");
    flow.rx_packet_count.fetch_add(1, Ordering::AcqRel);
    flow.rx_byte_count.fetch_add(1, Ordering::AcqRel);

    Ok(())
}

#[inline]
fn accel_tcp(skb: *const bindings::sk_buff, ip_header: &pdu::Ipv4Pdu, tcp_header: &pdu::TcpPdu) -> Result<(), AccelError> {
    let protocol = ip_header.protocol();
    let src_ip = u32::from_be_bytes(ip_header.source_address());
    let dest_ip = u32::from_be_bytes(ip_header.destination_address());
    let src_port = tcp_header.source_port();
    let dest_port = tcp_header.destination_port();

    let flow_key = FlowKey {
        dev: 0,
        protocol,
        src_ip,
        dest_ip,
        src_port,
        dest_port,
    };

    /* Lookup flow using key */
    let flow = do_flow_lookup(&flow_key)?;

    println!("Found SKB flow !!!");
    flow.rx_packet_count.fetch_add(1, Ordering::AcqRel);
    flow.rx_byte_count.fetch_add(1, Ordering::AcqRel);

    Ok(())
}

#[inline]
fn accel_ipv4(skb: *const bindings::sk_buff, packet_headers: &[u8]) -> Result<(), AccelError> {
    let ip_header = match pdu::Ip::new(packet_headers) {
        Ok(pdu::Ip::Ipv4(ipv4_pdu)) => {
            ipv4_pdu
        }
        Ok(pdu::Ip::Ipv6(ipv6_pdu)) => {
            println!("Wrong IPv6 packet {:?}", ipv6_pdu);
            return Err(AccelError::WrongProtocol);
        }
        Err(e) => {
            println!("Unknown packet protocol {:?}", e);
            return Err(AccelError::UnsupportedProtocol);
        }
    };

    match ip_header.inner() {
        Ok(pdu::Ipv4::Tcp(tcp)) => accel_tcp(skb, &ip_header, &tcp),
        Ok(pdu::Ipv4::Udp(udp)) => accel_udp(skb, &ip_header, &udp),
        _ => Err(AccelError::UnsupportedProtocol),
    }
}

#[no_mangle]
pub fn rust_accel_recv_ipv4(dev: *const bindings::net_device, skb: *const bindings::sk_buff, pkt_len: u32) -> c_types::c_int {
    /* First parse the packet headers */
    println!("Rust received network packet!");

    let packet_headers = unsafe {
        core::slice::from_raw_parts((*skb).data, pkt_len as usize)
    };

    match accel_ipv4(skb, packet_headers) {
        Ok(_) => 1,
        Err(_) => 0,
    }
}
