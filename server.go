package main

import (
	"fmt"
	"net/http"
	"time"

	"github.com/go-chi/chi"
	"github.com/go-chi/chi/middleware"
	"github.com/rs/cors"

	"github.com/Evanfeenstra/brood/frontend"
)

func server(port string, IS_DEV bool) *http.Server {
	r := initChi(IS_DEV)

	if !IS_DEV {
		r.Group(func(r chi.Router) {
			r.Get("/", frontend.IndexRoute)
			r.Get("/static/*", frontend.StaticRoute)
			r.Get("/manifest.json", frontend.ManifestRoute)
			r.Get("/favicon.ico", frontend.FaviconRoute)
		})
	}

	r.Group(func(r chi.Router) {
		r.Post("/check", checkWallet)
		r.Post("/create", createWallet)
		r.Post("/state", getState)
		r.Post("/faucet", faucet)
		r.Post("/send", send)
		r.Post("/coin", createCoin)
		r.Post("/register", registerCoin)
		r.Post("/clipboard", doClipboard)
	})

	srv := &http.Server{Addr: ":" + port, Handler: r}
	go func() {
		fmt.Println("Listening on port " + port)
		if err := srv.ListenAndServe(); err != nil {
			fmt.Println("Server startup error:", err.Error())
		}
	}()
	return srv
}

func initChi(IS_DEV bool) *chi.Mux {
	r := chi.NewRouter()
	r.Use(middleware.RequestID)
	r.Use(middleware.Recoverer)
	r.Use(middleware.Timeout(120 * time.Second))
	if IS_DEV { // dev comes from port 8000
		r.Use(middleware.Logger)
		cors := cors.New(cors.Options{
			AllowedOrigins:   []string{"http://localhost:8000"},
			AllowedMethods:   []string{"POST", "OPTIONS"},
			AllowedHeaders:   []string{"Accept", "Content-Type"},
			AllowCredentials: true,
			MaxAge:           300, // Maximum value not ignored by any of major browsers
			Debug:            false,
		})
		r.Use(cors.Handler)
	}
	return r
}
