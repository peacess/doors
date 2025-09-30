package main

import (
	"flag"
	"fmt"
	"io/fs"
	"os"
	"strings"

	"github.com/peacess/doors/doors_lib/go/gen_ffi_rpc/rust"
	"github.com/peacess/doors/doors_lib/go/go_lib"
)

func main() {
	fbsPaths := flag.String("fsb", "./", "fsb path, use commas to separate")
	outPath := flag.String("out", "./", "out put path, if dont exist, then create ")
	flag.Usage = func() {
		flag.PrintDefaults()
	}
	flag.Parse()

	fbs := strings.Split(*fbsPaths, ",")
	if len(fbs) < 1 {
		fmt.Println("parameter fsb is empty")
		return
	}
	for _, v := range fbs {
		if !golib.Files.IsDir(v) {
			//just warning
			fmt.Printf("warning,  fbs(%s) is not a dir\n", v)
		}
	}

	if !golib.Files.Exists(*outPath) {
		if err := os.MkdirAll(*outPath, fs.ModePerm); err != nil {
			fmt.Println(err)
			return
		}
	} else if golib.Files.IsFile(*outPath) {
		fmt.Println("the out path is a file, not a path")
		return
	}

	rust.GenRust(fbs, *outPath)

}
