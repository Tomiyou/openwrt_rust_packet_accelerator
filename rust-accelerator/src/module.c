#include <linux/kernel.h>
#include <linux/init.h>
#include <linux/module.h>
#include <linux/skbuff.h>
#include <net/addrconf.h>
#include <linux/inetdevice.h>

/*
 * Expose the hook for the receive processing.
 */
extern int (*athrs_fast_nat_recv)(struct sk_buff *skb);

/* Rust forward declarations */
extern int rust_init(void);
extern void rust_cleanup(void);
extern int rust_accel_recv_ipv4(struct net_device *dev, struct sk_buff *skb, unsigned int pkt_len);
extern int rust_accel_recv_ipv6(struct net_device *dev, struct sk_buff *skb, unsigned int pkt_len);

inline bool can_do_ipv4_accel(struct net_device *dev) {
    struct in_device *in4_dev;

    BUG_ON(!dev);

    /*
     * Is device part of a bridge?
     */
    if (dev->priv_flags & IFF_BRIDGE_PORT) {
        return true;
    }

    /*
     * Does our input device support IPv4 processing?
     */
    in4_dev = (struct in_device *)dev->ip_ptr;
    if (unlikely(!in4_dev)) {
        return false;
    }

    /*
     * Does it have an IPv4 address?  If it doesn't then we can't do anything
     * interesting here!
     */
    if (unlikely(!in4_dev->ifa_list)) {
        return false;
    }

    return true;
}

inline bool can_do_ipv6_accel(struct net_device *dev)
{
	struct inet6_dev *in6_dev;

	BUG_ON(!dev);

	/*
     * Is device part of a bridge?
     */
    if (dev->priv_flags & IFF_BRIDGE_PORT) {
        return true;
    }

	/*
	 * Does our input device support IPv6 processing?
	 */
	in6_dev = (struct inet6_dev *)dev->ip6_ptr;
	if (unlikely(!in6_dev)) {
		return false;
	}

	/*
	 * Does it have an IPv6 address?  If it doesn't then we can't do anything
	 * interesting here!
	 */
	if (unlikely(list_empty(&in6_dev->addr_list))) {
		return false;
	}

	return true;
}

int rust_accel_skb_recv(struct sk_buff *skb) {
    struct net_device *dev;

	/*
	 * We know that for the vast majority of packets we need the transport
	 * layer header so we may as well start to fetch it now!
	 */
	prefetch(skb->data + 32);
	barrier();

	dev = skb->dev;

// #ifdef CONFIG_NET_CLS_ACT
// 	/*
// 	 * If ingress Qdisc configured, and packet not processed by ingress Qdisc yet
// 	 * We can not accelerate this packet.
// 	 */
// #if (LINUX_VERSION_CODE < KERNEL_VERSION(5, 4, 0))
// 	if (dev->ingress_queue && !(skb->tc_verd & TC_NCLS)) {
// 		return 0;
// 	}
// #else
// 	if (rcu_access_pointer(dev->miniq_ingress) && !skb->tc_skip_classify) {
// 		return 0;
// 	}
// #endif
// #endif

	/*
	 * We're only interested in IPv4 and IPv6 packets.
	 */
	if (likely(htons(ETH_P_IP) == skb->protocol)) {
		if (can_do_ipv4_accel(dev)) {
			return rust_accel_recv_ipv4(dev, skb, skb->len);
		}
	}

	if (likely(htons(ETH_P_IPV6) == skb->protocol)) {
		if (can_do_ipv6_accel(dev)) {
			return rust_accel_recv_ipv6(dev, skb, skb->len);
		}
	}

	return 0;
}

int init_module(void)
{
    int error;

    printk("Hello from C!\n");

    // Init rust kernel module
    error = rust_init();
    if (error) {
        return error;
    }

    // Call kernel macros ...
    BUG_ON(athrs_fast_nat_recv);
	RCU_INIT_POINTER(athrs_fast_nat_recv, rust_accel_skb_recv);

    return 0;
}

void cleanup_module(void)
{
    // Call kernel macros ...
    RCU_INIT_POINTER(athrs_fast_nat_recv, NULL);

    rust_cleanup();

    printk("Goodbye from C!\n");
}

void bug_helper(void)
{
    BUG();
}

MODULE_DESCRIPTION("Rust port of Shortcut Forwarding Engine");
MODULE_AUTHOR("Tomaz Hribernik");
MODULE_LICENSE("GPL");
