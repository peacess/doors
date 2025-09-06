.PHONY: build clean format upgrade

format:
	cd app_egui && cargo +nightly fmt
	@echo "==> rust/app_egui Code formatted."
clean:
	cd app_egui && cargo clean
	@echo "==> rust/app_egui Code cleaned."
build:
	cd app_egui && cargo build --release
	@echo "==> rust/app_egui Code built."
upgrade:
	cd app_egui && cargo upgrade
	@echo "==> rust/app_egui Dependencies upgraded."