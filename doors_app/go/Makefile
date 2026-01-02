.PHONY: build clean format upgrade

go := ${shell which go}

format:
	cd doors && ${go} fmt ./...
clean:
	cd doors && rm -rf go.sum main doors_go build
build:
	cd doors && ${go} build -ldflags="-s -w" ./...
	cd doors && ${go} build -ldflags="-s -w" -o build/doors_go main.go
	@echo "==> rust/app_egui Code built."
upgrade:
	cd doors && ${go} get -u -t ./...
