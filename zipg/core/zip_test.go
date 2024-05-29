package core

import (
	"archive/zip"
	"fmt"
	"io"
	"io/fs"
	"os"
	"path/filepath"
	"strings"
	"sync"
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

func TestUnzip(t *testing.T) {
	zipFiles := []string{`C:\Users\aagao\Downloads\ts-practice.zip`, `C:\Users\aagao\Downloads\GhidraBookCode.zip`, `C:\Users\aagao\Downloads\sample_20240228.zip`}

	var wg sync.WaitGroup

	for _, zipFile := range zipFiles {
		wg.Add(1)
		go func(zipFile string) {
			defer wg.Done()
			if err := unzipFile(zipFile, "./output"); err != nil {
				fmt.Printf("Error unzipping file %s: %v\n", zipFile, err)
			} else {
				fmt.Printf("Successfully unzipped file %s\n", zipFile)
			}
		}(zipFile)
	}
	wg.Wait()
}

func unzipFile(zipFile, desDir string) error {
	r, err := zip.OpenReader(zipFile)
	if err != nil {
		return err
	}

	defer r.Close()

	for _, f := range r.File {
		filePath := filepath.Join(desDir, f.Name)
		if f.FileInfo().IsDir() {
			os.MkdirAll(filePath, os.ModePerm)
			continue
		}

		if err := os.MkdirAll(filepath.Dir(filePath), os.ModePerm); err != nil {
			return err
		}

		outFile, err := os.OpenFile(filePath, os.O_WRONLY|os.O_CREATE|os.O_TRUNC, f.Mode())

		if err != nil {
			return err
		}

		rc, err := f.Open()
		if err != nil {
			return err
		}

		_, err = io.Copy(outFile, rc)
		outFile.Close()
		rc.Close()

		if err != nil {
			return err
		}
	}

	return nil
}

func CheckErr(e error, t *testing.T) {
	if e != nil {
		t.Fatalf("%v", e)
	}
}

func TestZipDir(t *testing.T) {
	test := []string{`c:\temp\ga\*`, `C:\Users\aagao\Downloads\nmap-7.95-setup.exe`}

	outFile, err := os.Create("C:/temp/output.zip")
	if err != nil {
		t.Fatalf("%v", err)
	}

	defer outFile.Close()

	zipWriter := zip.NewWriter(outFile)
	defer zipWriter.Close()

	var wg sync.WaitGroup
	errChan := make(chan error, 10)

	for _, pattern := range test {
		mathches, err := filepath.Glob(pattern)
		CheckErr(err, t)

		for _, file := range mathches {
			t.Log("filename = ", file)
			if isDir(file) {
				err := filepath.Walk(file, func(path string, info fs.FileInfo, err error) error {
					if err != nil {
						return err
					}

					if info.IsDir() {
						return nil
					}
					err = addFileToZip(zipWriter, path, file)
					if err != nil {
						return err
					}

					return nil
				})

				if err != nil {
					errChan <- err
				}
			} else {
				wg.Add(1)
				func(file, pattern string) {
					defer wg.Done()
					err := addFileToZip(zipWriter, file, filepath.Dir(pattern))
					if err != nil {
						errChan <- err
					}
				}(file, pattern)
			}
		}
	}

	wg.Wait()
	close(errChan)

	if errChan != nil {
		fmt.Println("Received error:", errChan)
	}

}

func TestWalkDir(t *testing.T) {
	root := `C:\temp`

	err := filepath.WalkDir(root, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}
		t.Log("filename = ", d.Name(), d.IsDir())

		return nil
	})

	if err != nil {
		t.Fatalf("%v", err)
	}
}

func TestWalk(t *testing.T) {
	root := `C:\temp`
	err := filepath.Walk(root, func(path string, info fs.FileInfo, err error) error {
		if err != nil {
			return err
		}

		if info.IsDir() {
			return nil
		}

		relpath, err := filepath.Rel(root, path)
		if err != nil {
			return err
		}
		t.Log("filename = ", info.Name(), info.IsDir())
		t.Log("path = ", path)
		t.Log("rel", relpath, "\n")

		return nil
	})

	if err != nil {
		t.Fatalf("%v", err)
	}
}
