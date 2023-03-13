
# MAKE_OPTS := ARCH="$(ARCH)" CROSS_COMPILE="$(CROSS_COMPILE)" KSRC="$(KSRC)"

CARGO_TARGET := x86_64-linux-kernel-module

ifneq ($(IS_COMPILE_STAGE),)
# First stage: compile rust_accelerator.a
$(info Rust accelerator compile stage)

all: build_cargo_lib
	$(info Linking kernel module)
	@touch target/$(CARGO_TARGET)/release/.librust_accelerator.a.cmd
	$(MAKE) -C "$(CURDIR)" IS_COMPILE_STAGE= RELEASE=1

build_cargo_lib:
	$(info Calling cargo...)
	RUST_TARGET_PATH="$(CURDIR)/.." cargo +nightly build --release

clean:
	$(MAKE) -C "$(CURDIR)" IS_COMPILE_STAGE= RELEASE=1 clean

else
# Second stage: link lib_rust_accelerator.a into rust_accelerator.o -> rust_accelerator.ko
$(info Rust accelerator link stage)

obj-m := rust_accelerator.o
ifeq ($(RELEASE),1)
	BUILD = release
else
	BUILD = debug
endif
rust_accelerator-objs := ../src/c_wrapper.o src/module.o target/$(CARGO_TARGET)/$(BUILD)/librust_accelerator.a

EXTRA_LDFLAGS += --entry=init_module

$(info Blabla $(rust_accelerator-objs))

all:
	$(MAKE) -C "$(KSRC)" $(MAKE_OPTS) M="$(CURDIR)" V="s" modules

clean:
	$(MAKE) -C "$(KSRC)" $(MAKE_OPTS) M="$(CURDIR)" clean

endif
