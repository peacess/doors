.PHONY: build clean format upgrade

go := ${shell which go}

format:
	cd doors && ${go} fmt ./...

clean:
	cd doors && ${go} clean
build:
	cd doors && ${go} build -ldflags="-s -w" ./...

upgrade:
	cd doors && ${go} get -u -t ./...


