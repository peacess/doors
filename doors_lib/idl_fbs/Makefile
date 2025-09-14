.PHONY: clean gen golang rust dart

go := ${shell which go}

golang:
	flatc --go --go-module-name "github.com/peacess/doors/doors_lib/go/idl" -o ../go/idl base.fbs partner.fbs
rust:
	flatc --rust -o ../rust/idl/src/idl base.fbs partner.fbs
dart:
	flatc --dart -o ../dart/lib/idl base.fbs partner.fbs

clean:
	rm -rf ../go/idl/*
	rm -rf ../rust/dchat/src/idl/*
	rm -rf ../dart/lib/idl/*

gen:
	@$(MAKE) clean
	@$(MAKE) golang
	@$(MAKE) rust
	@$(MAKE) dart
	