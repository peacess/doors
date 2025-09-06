.PHONY: build clean format upgrade

format:
	cd flutter/doors && make format
	cd rust && make format

clean:
	cd flutter/doors && make clean
	cd rust && make clean

build:
	cd rust && make build
	cd flutter/doors && make build

upgrade:
	cd rust && make upgrade
	cd flutter/doors && make upgrade