package main

import (
	"context"
	"fmt"
	"net/http"
	"os"
	"time"

	"github.com/webview/webview"
)

func main() {

	var server *http.Server
	port := "3579"

	isDev := os.Getenv("DEV") // export DEV=true
	if isDev == "true" {
		port = "8000" // webpack dev server
	} else {
		server = srv(port) // prod server
	}

	debug := true
	w := webview.New(debug)

	defer func() {
		// cleanup webview
		w.Destroy()
		// cleanup http server
		if server != nil {
			ctx, cancel := context.WithTimeout(context.Background(), 3*time.Second)
			defer cancel()
			if err := server.Shutdown(ctx); err != nil {
				fmt.Printf("error shutting down server: %s", err.Error())
			}
		}
	}()

	w.SetTitle("Minimal webview example")
	w.SetSize(999, 650, webview.HintNone)
	w.Navigate("http://localhost:" + port)
	w.Run()
}

func connect() {
	loadConfig()
	// load wallet
	wallet := loadWallet()
	defer writeWalletStateFile(wallet, "wallet.dat")

	confirmedBalance, pendingBalance, err := wallet.Balance()
	if err != nil {
		fmt.Println(err)
	}

	fmt.Printf("confirmed: %+v\n", confirmedBalance)
	fmt.Printf("pending: %+v\n", pendingBalance)
}
