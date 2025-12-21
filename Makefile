.PHONY: build clean gen format upgrade install

flutter := ${shell which flutter}

format:
	cd doors_app && make format
	cd doors_lib && make format

clean:
	cargo clean
	${flutter} clean
	cd doors_app && make clean
	cd doors_lib && make clean
gen:
	cd doors_lib/idl_fbs && make gen
	cd doors_lib && make gen
	cd doors_app && make gen

build: gen
	cd doors_app && make build
	cd doors_lib && make build

upgrade:
	cd doors_app && make upgrade
	cd doors_lib && make upgrade
	cargo upgrade && flutter pub upgrade

install:
	mkdir "temp"
	cd temp
	wget -o temp_flatc.zip https://github.com/google/flatbuffers/releases/download/v25.2.10/Linux.flatc.binary.clang++-18.zip
	unzip -o temp_flatc.zip && chmod +x ./flatc
	sudo cp -f flatc /usr/local/bin/flatc
	cd ../
	rm -rf ./temp
