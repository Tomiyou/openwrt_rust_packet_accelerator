#include "bindgen_helper.h"

/* Global variables */
const gfp_t RUST_GFP_KERNEL = GFP_KERNEL;

/* Spinlock */
void spin_lock_init_wrapper(spinlock_t *lock) {spin_lock_init(lock); }
void spin_lock_wrapper(spinlock_t *lock) { spin_lock(lock); }
void spin_unlock_wrapper(spinlock_t *lock) { spin_unlock(lock); }

/* Mutex */
void mutex_init_wrapper(struct mutex *lock) { mutex_init(lock); }
void mutex_lock_wrapper(struct mutex *lock) { mutex_lock(lock); }
void mutex_unlock_wrapper(struct mutex *lock) { mutex_unlock(lock); }
