package core

import (
	"archive/zip"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"strings"
	"sync"

	"github.com/spf13/cobra"
)

func ConvertToZip(zipFileName string, filePaths []string) {
	zipFile, err := os.Create(zipFileName)
	cobra.CheckErr(err)
	defer zipFile.Close()
	zipWriter := zip.NewWriter(zipFile)
	defer zipWriter.Close()

	for _, filePath := range filePaths {
		matches, err := filepath.Glob(filePath)
		cobra.CheckErr(err)

		for _, matchedFilePath := range matches {
			file, err := os.Stat(matchedFilePath)
			cobra.CheckErr(err)

			if file.IsDir() {
				err := dirToZip(matchedFilePath, zipWriter)
				cobra.CheckErr(err)
			} else {
				fileToZip(matchedFilePath, zipWriter)
			}
		}
	}
}

func dirToZip(filePath string, zipWriter *zip.Writer) error {
	files, err := os.ReadDir(filePath)
	cobra.CheckErr(err)

	for _, file := range files {
		//wg.Add(1)
		fileToZip(filepath.Join(filePath, file.Name()), zipWriter)
	}
	return nil
}

func fileToZip(file string, zipWriter *zip.Writer) {
	//defer wg.Done()
	srcFile, err := os.Open(file)
	//srcFile, err := os.ReadFile(file)
	cobra.CheckErr(err)

	defer srcFile.Close()
	zipEntry, err := zipWriter.Create(filepath.Base(file))
	cobra.CheckErr(err)

	_, err = io.Copy(zipEntry, srcFile)
	cobra.CheckErr(err)
}

func UnZip(zipFilePath string, dstPath string) {
	zipReader, err := zip.OpenReader(zipFilePath)
	cobra.CheckErr(err)
	defer zipReader.Close()

	var wg sync.WaitGroup

	for _, file := range zipReader.File {
		wg.Add(1)

		go func(zipFile *zip.File) {
			defer wg.Done()
			extratFilePath := fmt.Sprintf("%s/%s", dstPath, zipFile.Name)

			mkdirAll(extratFilePath)
			if !zipFile.FileInfo().IsDir() && dstPath == "" {
				extratFilePath = zipFile.Name
				extractFile(zipFile, extratFilePath)
			} else if !zipFile.FileInfo().IsDir() {
				extractFile(zipFile, extratFilePath)
			}
		}(file)
	}

	wg.Wait()
}

func extractFile(file *zip.File, dstPath string) {
	zipFile, err := file.Open()
	cobra.CheckErr(err)
	defer zipFile.Close()

	dstFile, err := os.Create(dstPath)
	cobra.CheckErr(err)
	defer dstFile.Close()

	_, err = io.Copy(dstFile, zipFile)
	cobra.CheckErr(err)
}

/*
filePath = "rust/go/cpp/main.cs"
mkdir "rust/go/cpp"	フォルダを作成。
*/
func mkdirAll(filePath string) {
	if !strings.HasSuffix(filePath, "/") {
		filePath = filepath.Dir(filePath)
	}
	os.MkdirAll(filePath, os.ModePerm)
}
