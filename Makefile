.PHONY: build clean gen format upgrade install

flutter := ${shell which flutter}

format:
	cd doors_app && make format
	cd doors_lib && make format

clean:
	cd doors_app && make clean
	cd doors_lib && make clean
	cargo clean && rm -f Cargo.lock
	${flutter} clean && rm -f pubspec.lock
gen:
	cd doors_lib/idl_fbs && make gen
	cd doors_lib && make gen
	cd doors_app && make gen

build: gen
	dart pub get
	cd doors_app && make build
	cd doors_lib && make build

upgrade:
	cd doors_app && make upgrade
	cd doors_lib && make upgrade
	cargo upgrade && flutter pub upgrade

install:
	mkdir -p temp
	cd temp && wget -O temp_flatc.zip https://github.com/google/flatbuffers/releases/download/v25.12.19/Linux.flatc.binary.clang++-18.zip
	cd temp && unzip -o temp_flatc.zip && chmod +x ./flatc
	sudo cp -f temp/flatc /usr/local/bin/flatc
	rm -rf ./temp
	flatc --version
