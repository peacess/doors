.PHONY: build clean format upgrade

format:
	cd app_egui && cargo +nightly fmt
clean:
	cd app_egui && cargo clean
build:
	cd app_egui && cargo build --release
	@echo "==> rust/app_egui Code built."
upgrade:
	cd app_egui && cargo upgrade
