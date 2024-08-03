package core

import (
	"fmt"
	"log"
	"net"
	"net/http"
	"net/http/httputil"
	"os"
	"path/filepath"
	"strconv"
)

type CoffeeHttpServer struct {
	dir  string
	port int
}

func NewCoffeeHttpServer(dir string, port int) *CoffeeHttpServer {
	return &CoffeeHttpServer{
		dir:  dir,
		port: port,
	}
}

func (c *CoffeeHttpServer) fileDownloadHandler(w http.ResponseWriter, r *http.Request) {
	dumpRequest(r)

	fileName := r.URL.Query().Get("file")
	if fileName == "" {
		http.Error(w, "Not Found File", http.StatusNotFound)
		return
	}

	filePath := filepath.Join(c.dir, fileName)
	file, err := os.Open(filePath)
	if err != nil {
		http.Error(w, "Not Found File", http.StatusNotFound)
		return
	}

	defer file.Close()

	w.Header().Set("Content-Disposition", fmt.Sprintf("attachment; filename=\"%s\"", fileName))
	w.Header().Set("Content-Type", "application/octet-stream")
	http.ServeFile(w, r, filePath)
}

func (c *CoffeeHttpServer) indexHandler(w http.ResponseWriter, r *http.Request) {
	dumpRequest(r)

	files, err := os.ReadDir(c.dir)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
	}

	w.Header().Set("Content-Type", "text/html; chaset=utf-8")
	fmt.Fprintf(w, "<html><body><h1>File Lists</h1><ul>")

	for _, file := range files {
		if !file.IsDir() {
			fileName := file.Name()
			fmt.Fprintf(w, "<li><a href=\"/download?file=%s\">%s</a></li>", fileName, fileName)
		}
	}

	fmt.Fprintf(w, "</ul></body></html>")
}

func (c *CoffeeHttpServer) Run() {
	http.HandleFunc("/", c.indexHandler)
	http.HandleFunc("/download", c.fileDownloadHandler)

	addr := getPrivateIpAddr()

	fmt.Printf("http://%s:%s", addr, strconv.Itoa(c.port))

	err := http.ListenAndServe(
		":"+strconv.Itoa(c.port),
		nil,
	)

	if err != nil {
		log.Fatalf("%v\n", err)
		os.Exit(1)
	}
}

func getPrivateIpAddr() string {
	defaultHost := "localhost"

	conn, err := net.Dial("udp", "8.8.8.8:80")
	if err != nil {
		fmt.Println(err)
		return defaultHost
	}
	defer conn.Close()

	localAddr := conn.LocalAddr().(*net.UDPAddr)

	return localAddr.IP.String()
}

func dumpRequest(r *http.Request) {
	info, err := httputil.DumpRequest(r, true)
	if err != nil {
		fmt.Printf("%v\n", err)
	}
	fmt.Printf("%s\n", string(info))
}
