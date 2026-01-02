.PHONY: build clean gen format upgrade install

flutter := ${shell which flutter}
go := ${shell which go}

format:
	cd doors_app && make format
	cd doors_lib && make format

clean:
	${go} clean
	cd doors_app && make clean
	cd doors_lib && make clean
	cargo clean && rm -rf Cargo.lock target
	${flutter} clean && rm -f pubspec.lock build
	rm -rf go.work.sum
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

install_cargo:
	cargo install cargo-edit

install_flatc:
	mkdir -p temp
	cd temp && wget -O temp_flatc.zip https://github.com/google/flatbuffers/releases/download/v25.12.19/Linux.flatc.binary.clang++-18.zip
	cd temp && unzip -o temp_flatc.zip && chmod +x ./flatc
	sudo cp -f temp/flatc /usr/local/bin/flatc
	rm -rf ./temp
	flatc --version
install_go:
	sudo apt install libxxf86vm-dev

install_zig:
	${go} install github.com/asdf-vm/asdf/cmd/asdf@latest
	#add "$HOME/.asdf/shims" to your PATH environment variable
	asdf plugin add zig https://github.com/asdf-community/asdf-zig.git
	asdf plugin update zig
	asdf install zig latest
	asdf set --home zig latest
	zig version
	# curl -L https://github.com/marler8997/zigup/releases/download/v2025_05_24/zigup-x86_64-linux.tar.gz | tar xz
	# sudo mv zigup /usr/local/bin
	# zigup 0.15.2
	# zig version
