.PHONY: build clean format upgrade

go := ${shell which go}
flutter := ${shell which flutter}

format:
	cd go && ${go} fmt ./...
	cd dart && dart format ./ -l 160
	cd rust/dchat && cargo +nightly fmt
	cd rust/idl && cargo +nightly fmt
	cd zig/ffi_rpc && zig fmt ./

clean:
	cd go && ${go} clean
	cd dart && ${flutter} clean
	cd rust/dchat && cargo clean
	cd rust/idl && cargo clean
	cd idl_fbs && make clean
build:
	cd go && ${go} build -ldflags="-s -w" ./...
	cd rust/dchat && cargo build --release
	cd rust/idl && cargo build --release
	${flutter} analyze

upgrade:
	cd go && ${go} get -u -t ./...
	cd rust/dchat && cargo upgrade
	cd rust/idl && cargo upgrade


	