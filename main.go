package main

import (
	"context"
	"fmt"
	"os"
	"time"

	"github.com/webview/webview"
)

func main() {

	port := "3579"

	isDev := os.Getenv("DEV") == "true" // export DEV=true
	server := srv(port, isDev)

	debug := true
	w := webview.New(debug)

	defer func() {
		// cleanup webview
		w.Destroy()
		// cleanup http server
		ctx, cancel := context.WithTimeout(context.Background(), 3*time.Second)
		defer cancel()
		if err := server.Shutdown(ctx); err != nil {
			fmt.Printf("error shutting down server: %s", err.Error())
		}
	}()

	w.SetTitle("Brood Wallet")
	w.SetSize(888, 646, webview.HintNone)

	appPort := port
	if isDev {
		appPort = "8000"
	}
	w.Navigate("http://localhost:" + appPort)
	w.Run()
}
