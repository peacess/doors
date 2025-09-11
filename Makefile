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
	mkdir "temp"
	cd temp
	wget -o temp_flatc.zip https://github.com/google/flatbuffers/releases/download/v25.2.10/Linux.flatc.binary.clang++-18.zip
	unzip -o temp_flatc.zip && chmod +x ./flatc
	sudo cp -f flatc /usr/local/bin/flatc
	cd ../
	rm -rf ./temp