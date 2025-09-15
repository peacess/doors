.PHONY: clean gen golang rust dart kotlin

go := ${shell which go}

golang:
	flatc --go --go-module-name "github.com/peacess/doors/doors_lib/go/idl" -o ../go/idl base.fbs partner.fbs
rust:
	flatc --rust -o ../rust/idl/src/idl base.fbs partner.fbs
dart:
	#flatc --dart --gen-object-api -o ../dart/lib/idl base.fbs partner.fbs
	flatc --dart -o ../dart/lib/idl base.fbs partner.fbs

kotlin:
	flatc --kotlin -o ../kotlin/src/idl base.fbs partner.fbs

clean:
	rm -rf ../go/idl/*
	rm -rf ../rust/dchat/src/idl/*_generated.rs
	rm -rf ../dart/lib/idl/*_generated.dart

gen:
	@$(MAKE) clean
	@$(MAKE) golang
	@$(MAKE) rust
	@$(MAKE) dart
	