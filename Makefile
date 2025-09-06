.PHONY: build clean format upgrade install

format:
	cd doors_app && make format
	cd doors_lib && make format

clean:
	cargo clean
	cd doors_app && make clean
	cd doors_lib && make clean
build:
	cd doors_app && make build
	cd doors_lib && make build

upgrade:
	cd doors_app && make upgrade
	cd doors_lib && make upgrade

install:
	sudo apt install -y flatbuffers-compiler