extern crate alloc;
use linux_kernel_module::{c_types, println, bindings};
use core::sync::atomic::{AtomicU32, Ordering};

#[no_mangle]
pub fn rust_accel_recv_ipv6(dev: *const bindings::net_device, skb: *const bindings::sk_buff) -> c_types::c_int {
    println!("Rust received network packet!");

    let global = crate::get_global_data();

    // let flow_key = crate::ipv4::FlowKey::default();

    // let flow = {
    //     let guard = global.ipv4_flows.lock();

    //     let flow = match guard.get(&flow_key) {
    //         Some(flow) => flow,
    //         None => return 0,
    //     };

    //     flow.clone()
    // };

    // println!("Found SKB flow !!!");
    // flow.rx_packet_count.fetch_add(1, Ordering::AcqRel);
    // flow.rx_byte_count.fetch_add(1, Ordering::AcqRel);

    return 0;
}