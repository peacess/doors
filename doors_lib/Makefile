.PHONY: build clean format upgrade

go := ${shell which go}

format:
	cd go && ${go} fmt ./...
	cd rust/dchat && cargo +nightly fmt

clean:
	cd go && ${go} clean
	cd rust/dchat && cargo clean
build:
	cd go && ${go} build  ./...
	cd rust/dchat && cargo build --release

upgrade:
	cd go && ${go} get -u -t ./...
	cd rust/dchat && cargo upgrade