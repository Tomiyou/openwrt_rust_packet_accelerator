#include <linux/kernel.h>
#include <linux/init.h>
#include <linux/module.h>
#include <linux/skbuff.h>

/*
 * Expose the hook for the receive processing.
 */
extern int (*athrs_fast_nat_recv)(struct sk_buff *skb);

/* Rust forward declarations */
extern int rust_init(void);
extern void rust_cleanup(void);
extern int rust_skb_recv(struct sk_buff *skb);

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
	RCU_INIT_POINTER(athrs_fast_nat_recv, rust_skb_recv);

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
