.PHONY: build clean format

flutter := ${shell which flutter}

format:
	dart format ./ -l 160
clean:
	${flutter} clean
build:
	${flutter} analyze