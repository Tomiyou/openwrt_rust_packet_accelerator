include $(TOPDIR)/rules.mk
include $(INCLUDE_DIR)/kernel.mk

PKG_NAME:=rust-accelerator
PKG_VERSION=0.1.0

# PKG_SOURCE_PROTO:=git
# PKG_SOURCE_URL:=https://github.com/Tomiyou/openwrt_rust_packet_accelerator.git
# PKG_SOURCE_VERSION:=d79fe1fce246c6c033f1ffa335a8da432adb73a2
# PKG_SOURCE_SUBDIR:=$(PKG_NAME)-$(PKG_VERSION)
# PKG_SOURCE:=$(PKG_NAME)-$(PKG_VERSION)-$(PKG_SOURCE_VERSION).tar.gz

include $(INCLUDE_DIR)/local-development.mk
include $(INCLUDE_DIR)/package.mk

define KernelPackage/rust-accelerator
  SECTION:=kernel
  CATEGORY:=Kernel modules
  SUBMENU:=Network Support
  TITLE:=Packet forwarding accelerator written in Rust
  FILES:=$(PKG_BUILD_DIR)/rust-accelerator/rust_accelerator.ko
endef

define KernelPackage/rust-accelerator/description
	rust-accelerator Rust kernel module.
endef

MAKE_OPTS := \
		CARGO_TARGET="armv7a-linux-kernel-module" \
		RELEASE="1" \
		ARCH="$(LINUX_KARCH)" \
		CROSS_COMPILE="$(TARGET_CROSS)" \
		KSRC="$(LINUX_DIR)"

define Build/Prepare
	cp -lr ../../../../rust_accelerator/. $(PKG_BUILD_DIR)
endef

define Build/Compile
	$(MAKE) \
		-C "$(PKG_BUILD_DIR)/rust-accelerator" \
		$(MAKE_OPTS)
endef

$(eval $(call KernelPackage,rust-accelerator))
