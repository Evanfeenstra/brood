package main

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"net/http"

	"github.com/atotto/clipboard"
)

type ClipboardRes struct {
	Cmd  string `json:"cmd"`
	Text string `json:"text"`
	Meta string `json:"meta"`
}

func doClipboard(w http.ResponseWriter, r *http.Request) {
	bod := ClipboardRes{}
	body, err := ioutil.ReadAll(r.Body)
	r.Body.Close()
	err = json.Unmarshal(body, &bod)
	if err != nil {
		fmt.Println(err)
		w.WriteHeader(http.StatusNotAcceptable)
		return
	}
	if bod.Cmd == "" {
		w.WriteHeader(http.StatusNotAcceptable)
		return
	}

	retText := ""
	if bod.Cmd == "copy" {
		clipboard.WriteAll(bod.Text)
	}

	if bod.Cmd == "paste" {
		retText, _ = clipboard.ReadAll()
	}

	w.WriteHeader(http.StatusOK)
	json.NewEncoder(w).Encode(ClipboardRes{
		Text: retText,
		Cmd:  bod.Cmd,
		Meta: bod.Meta,
	})
}
