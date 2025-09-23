.PHONY: build clean format install gen

flutter := ${shell which flutter}

format:
	dart format ./ -l 160
clean:
	${flutter} clean
build:
	${flutter} analyze
gen:
	${flutter} gen-l10n
	${flutter} pub run build_runner build
install:
# 	dart pub global activate flutter_gen