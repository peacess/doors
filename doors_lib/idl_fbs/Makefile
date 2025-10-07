.PHONY: clean gen golang rust dart kotlin

golang:
	flatc --go --go-module-name "github.com/peacess/doors/doors_lib/go/idl" -o ../go/idl ./*.fbs
rust:
	flatc --rust -o ../rust/idl/src/idl ./*.fbs
dart:
	#flatc --dart --gen-object-api -o ../dart/lib/idl base.fbs partner.fbs
	flatc --dart -o ../dart/lib/idl ./*.fbs

kotlin:
	flatc --kotlin -o ../kotlin/src/idl ./*.fbs

clean:
	rm -rf ../go/idl/*
	rm -rf ../kotlin/src/idl/*
	rm -rf ../rust/dchat/src/idl/*_generated.rs
	rm -rf ../dart/lib/idl/*_generated.dart

gen: clean golang rust dart kotlin
	