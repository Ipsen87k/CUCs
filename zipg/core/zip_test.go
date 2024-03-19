package core

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
)

func TestMkdirAll(t *testing.T) {
	path := "rust/go/main.c"
	if strings.HasSuffix(path, "/") {

	} else {
		path = filepath.Dir(path)
	}
	os.MkdirAll(path, os.ModePerm)

}
