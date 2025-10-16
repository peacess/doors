.PHONY: build clean format upgrade gen

format:
	cd flutter/doors && make format
	cd rust && make format
	cd go && make format

clean:
	cd flutter/doors && make clean
	cd rust && make clean
	cd go && make clean

build:
	cd rust && make build
	cd flutter/doors && make build
	cd go && make build

upgrade:
	cd rust && make upgrade
	cd flutter/doors && make upgrade
	cd go && make upgrade

gen:
	cd flutter/doors && make gen