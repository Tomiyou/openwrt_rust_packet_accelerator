
CARGO_TARGET ?= x86_64-linux-kernel-module
CARGO_OPTS := --target $(CARGO_TARGET)
KSRC ?= /lib/modules/$(shell uname -r)/build

# Cargo needs KSRC variable to generate kernel bindings
export KSRC

# Handle release builds
ifeq ($(RELEASE),1)
	BUILD = release
	CARGO_OPTS += --release
else
	BUILD = debug
endif

# Kernel object definitions
obj-m := rust_accelerator.o
rust_accelerator-objs := src/module.o ../src/c_wrapper.o  target/$(CARGO_TARGET)/$(BUILD)/librust_accelerator.a

# Recipes
all: build_cargo_lib
	$(MAKE) -C "$(KSRC)" M="$(CURDIR)" modules

clean:
	RUST_TARGET_PATH="$(CURDIR)/.." cargo +nightly clean $(CARGO_OPTS)
	$(MAKE) -C "$(KSRC)" M="$(CURDIR)" clean

build_cargo_lib:
	$(info Calling cargo...)
	RUST_TARGET_PATH="$(CURDIR)/.." cargo +nightly build $(CARGO_OPTS)
	@touch target/$(CARGO_TARGET)/$(BUILD)/.librust_accelerator.a.cmd
