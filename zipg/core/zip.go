package core

import (
	"archive/zip"
	"io"
	"io/fs"
	"os"
	"path/filepath"
	"strings"

	"github.com/spf13/cobra"
)

func ZipFiles(zipFilename string, patterns []string) error {
	outFile, err := os.Create(zipFilename)
	if err != nil {
		return err
	}
	defer outFile.Close()

	zipWriter := zip.NewWriter(outFile)
	defer zipWriter.Close()

	fileChan := make(chan string, 10)
	errChan := make(chan error, 10)

	for _, pattern := range patterns {
		matches, err := filepath.Glob(pattern)
		if err != nil {
			return err
		}

		for _, file := range matches {
			if !isDir(file) {
				func(file, pattern string) {

					err := addFileToZip(zipWriter, file, filepath.Dir(pattern))
					if err != nil {
						errChan <- err
					}
					fileChan <- file
				}(file, pattern)
			} else {
				err := filepath.WalkDir(file, func(path string, d fs.DirEntry, err error) error {
					if err != nil {
						return err
					}

					if d.IsDir() {
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
			}
		}
	}

	close(fileChan)
	close(errChan)

	if len(errChan) > 0 {
		return <-errChan
	}

	return nil
}

func addFileToZip(zipWriter *zip.Writer, filename, baseDir string) error {
	file, err := os.Open(filename)
	if err != nil {
		return err
	}
	defer file.Close()

	relativePath, err := filepath.Rel(baseDir, filename)
	if err != nil {
		return err
	}

	fileInfo, err := file.Stat()
	if err != nil {
		return err
	}

	zipHeader, err := zip.FileInfoHeader(fileInfo)
	if err != nil {
		return err
	}

	zipHeader.Name = relativePath
	zipHeader.Method = zip.Deflate

	writer, err := zipWriter.CreateHeader(zipHeader)
	if err != nil {
		return err
	}

	_, err = io.Copy(writer, file)
	if err != nil {
		return err
	}

	return nil
}

func isDir(path string) bool {
	info, err := os.Stat(path)
	if err != nil {
		return false
	}
	return info.IsDir()
}

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

func UnZip(zipFile, dstPath string) error {
	r, err := zip.OpenReader(zipFile)
	if err != nil {
		return err
	}

	defer r.Close()

	for _, f := range r.File {
		filePath := filepath.Join(dstPath, f.Name)
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
