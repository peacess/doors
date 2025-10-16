.PHONY: build clean format upgrade

format:
	cd dchat && cargo +nightly fmt
	cd idl && cargo +nightly fmt
	cd ffi_rpc && cargo +nightly fmt

clean:
	cd dchat && cargo clean
	cd idl && cargo clean
	cd ffi_rpc && cargo clean
build:
	cd dchat && cargo build --release
	cd idl && cargo build --release
	cd ffi_rpc && cargo build --release

upgrade:
	cd dchat && cargo upgrade
	cd idl && cargo upgrade
	cd ffi_rpc && cargo upgrade

OS_NAME=$(shell uname | tr '[:upper:]' '[:lower:]')
ANDROID_AARCH64_LINKER=$(ANDROID_NDK)/toolchains/llvm/prebuilt/$(OS_NAME)-x86_64/bin/aarch64-linux-android35-clang
ANDROID_ARMV7_LINKER=$(ANDROID_NDK)/toolchains/llvm/prebuilt/$(OS_NAME)-x86_64/bin/armv7a-linux-androideabi35-clang
ANDROID_X86_64_LINKER=$(ANDROID_NDK)/toolchains/llvm/prebuilt/$(OS_NAME)-x86_64/bin/x86_64-linux-android35-clang
PATH := $(ANDROID_NDK)/toolchains/llvm/prebuilt/$(OS_NAME)-x86_64/bin:$(PATH)
target=../../target
dart_android=../dart/android/src/main/jniLibs

all:release_linux debug_linux

release_linux:
	cd ffi_rpc && cargo build --release
	cp -f ${target}/release/libffi_rpc.so ../dart/linux/
debug_linux:
	cd ffi_rpc && cargo build
	cp -f ${target}/debug/libffi_rpc.so ../dart/linux/
release_android:
	cd ffi_rpc && CC_aarch64_linux_android=$(ANDROID_AARCH64_LINKER) \
		CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER=$(ANDROID_AARCH64_LINKER) \
		cargo build --target aarch64-linux-android --release
	cp -f ${target}/aarch64-linux-android/release/libffi_rpc.so ${dart_android}/arm64-v8a/
	@echo "[DONE] $@"

	cd ffi_rpc && CC_armv7_linux_androideabi=$(ANDROID_ARMV7_LINKER) \
		CARGO_TARGET_ARMV7_LINUX_ANDROIDEABI_LINKER=$(ANDROID_ARMV7_LINKER) \
		cargo build --target armv7-linux-androideabi --release
	cp -rf ${target}/armv7-linux-androideabi/release/libffi_rpc.so ${dart_android}/armeabi-v7a/
	@echo "[DONE] $@"

	cd ffi_rpc && CC_x86_64_linux_android=$(ANDROID_X86_64_LINKER) \
		CARGO_TARGET_X86_64_LINUX_ANDROID_LINKER=$(ANDROID_X86_64_LINKER) \
		cargo build --target x86_64-linux-android --release
	cp -f ${target}/x86_64-linux-android/release/libffi_rpc.so ${dart_android}/x86_64/
	@echo "[DONE] $@"

.PHONY: init
init:
	rustup target add aarch64-apple-ios x86_64-apple-ios
	rustup target add aarch64-apple-darwin x86_64-apple-darwin
	rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android
	@if [ $$(uname) == "Darwin" ] ; then cargo install cargo-lipo ; fi
	cargo install cbindgen