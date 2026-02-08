.PHONY: build clean format upgrade gen

go := ${shell which go}
flutter := ${shell which flutter}

format:
	cd go && ${go} fmt ./...
	cd dart && dart format ./ -l 160
	cd rust && make format
	cd zig/ffi_rpc && zig fmt ./

clean:
	cd go && rm -rf go.sum go_lin/go.sum gen_ffi_rpc/go.sum build
	cd dart && ${flutter} clean
	cd rust && make clean
	cd idl_fbs && make clean
build:
	cd go && ${go} build -ldflags="-s -w" ./...
	cd go && ${go} build -ldflags="-s -w" -o build/chat_server ./cmd/chat_server/main.go
	cd rust && make build
	cd dart && ${flutter} analyze
	cd zig/ffi_rpc && zig build

upgrade:
	cd go && ${go} get -u -t ./...
	cd rust && make upgrade
	cd dart && ${flutter} pub upgrade --major-versions

gen:
	cd rust/ffi_rpc && cargo build --bin build_rs
	cd dart && dart run ffigen --config=ffigen.yaml
