#include <linux/kernel.h>
#include <linux/init.h>
#include <linux/module.h>

/* Rust forward declarations */
extern int rust_init(void);
extern void rust_cleanup(void);

/* Global variables */
const gfp_t RUST_GFP_KERNEL = GFP_KERNEL;

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

    return 0;
}

void cleanup_module(void)
{
    rust_cleanup();
}

MODULE_DESCRIPTION("Rust port of Shortcut Forwarding Engine");
MODULE_AUTHOR("Tomaz Hribernik");
MODULE_LICENSE("GPL");
