package main

import (
	"context"
	"fmt"
	"os"
	"time"

	"github.com/webview/webview"

	"github.com/Evanfeenstra/brood/wallet"
)

func main() {

	port := "3888"

	IS_DEV := os.Getenv("DEV") == "true" // export DEV=true
	IS_WEB := os.Getenv("WEB") == "true"
	srv := wallet.Server(port, IS_DEV)

	defer func() {
		// cleanup http server
		ctx, cancel := context.WithTimeout(context.Background(), 3*time.Second)
		defer cancel()
		if err := srv.Shutdown(ctx); err != nil {
			fmt.Printf("error shutting down server: %s", err.Error())
		}
	}()

	debug := true
	w := webview.New(debug)
	defer w.Destroy()

	w.SetTitle("Brood Wallet")
	w.SetSize(888, 646, webview.HintNone)

	appPort := port
	if IS_DEV {
		appPort = "8000"
	}
	if IS_WEB {
		prt := os.Getenv("PORT")
		if prt != "" {
			appPort = prt
		}
	}
	w.Navigate("http://localhost:" + appPort)
	w.Run()
}
