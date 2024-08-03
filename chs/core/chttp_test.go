package core

import (
	"testing"
)

func TestGetIpAddr(t *testing.T) {
	privateAddr := getPrivateIpAddr()
	t.Log(privateAddr)

}
