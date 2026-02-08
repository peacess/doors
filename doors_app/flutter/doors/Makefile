.PHONY: build clean format install gen

flutter := ${shell which flutter}

format:
	dart format ./ -l 160
clean:
	${flutter} clean
build:
	${flutter} analyze
release_linux:
	${flutter} build linux --release
debug_linux:
	${flutter} build linux --debug
gen:
	${flutter} gen-l10n
	${flutter} pub run build_runner build
gen_icon:
	dart run icons_launcher:create
install:
# 	dart pub global activate flutter_gen
upgrade:
	${flutter} pub upgrade --major-versions
