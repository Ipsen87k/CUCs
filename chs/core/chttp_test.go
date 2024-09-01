package core

import (
	"fmt"
	"io"
	"net/http"
	"os"
	"strings"
	"testing"
)

type TestCase struct {
	status string
	text   string
}

func NewTestCaseError() TestCase {

	return TestCase{status: "400", text: ""}
}

func GetHttp(url string) (TestCase, error) {
	resp, err := http.Get(url)
	if err != nil {
		return NewTestCaseError(), err
	}
	defer resp.Body.Close()

	b, err := io.ReadAll(resp.Body)
	if err != nil {
		return NewTestCaseError(), err
	}

	test := TestCase{
		status: resp.Status,
		text:   string(b),
	}

	return test, nil
}

func TestGetIpAddr(t *testing.T) {
	privateAddr := getPrivateIpAddr()
	t.Log(privateAddr)

}

func TestHttpServer(t *testing.T) {
	server := NewCoffeeHttpServer("..", 8000)
	go func() {
		server.Run()
	}()
	url := "http://localhost:8000/"
	testCase, err := GetHttp(url)
	if err != nil {
		t.Fatalf("%v\n", err)
	}

	//Http Response Test
	expected, err := os.ReadFile("./expected/TestHttpServer.out")
	if err != nil {
		t.Fatalf("%v\n", err)
	}
	expectedStr := string(expected)

	if strings.Compare(testCase.status, "200 OK") != 0 {
		t.Fatalf("Not Status 200\n")
	}

	if strings.Compare(expectedStr, testCase.text) != 0 {
		t.Logf("testCase.text = %s", testCase.text)
		t.Fatalf("文字列が違います\n")
	}

	//File Download Test
	expected, err = os.ReadFile("expected/fileDownloadGoSum.out")
	if err != nil {
		t.Fatalf("%v\n", err)
	}
	expectedStr = string(expected)

	testCase, err = GetHttp(fmt.Sprintf("%sdownload?file=go.mod", url))
	if err != nil {
		t.Fatalf("%v\n", err)
	}

	if strings.Compare(expectedStr, testCase.text) != 0 {
		t.Logf("testCase.text = %s", testCase.text)
		t.Fatalf("文字列が違います\n")
	}

}
