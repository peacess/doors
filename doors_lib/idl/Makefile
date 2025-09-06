.PHONY: build clean format upgrade

go := ${shell which go}

golang:
	flatc  -g -o ../go base.fbs partner.fbs
rust:
	flatc --rust --gen-onefile -o ../rust/dchat/src/idl base.fbs partner.fbs