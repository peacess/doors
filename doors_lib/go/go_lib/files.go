package golib

import "os"

var Files files

type files struct{}

func (f files) Exists(file string) bool {
	_, err := os.Stat(file)
	if err != nil && os.IsNotExist(err) {
		return false
	}
	return true
}

func (f files) IsDir(file string) bool {
	stat, err := os.Stat(file)
	if err == nil {
		return stat.IsDir()
	}
	return false
}

func (f files) IsFile(file string) bool {
	stat, err := os.Stat(file)
	if err != nil {
		return false
	}

	return !stat.IsDir()
}
