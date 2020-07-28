package main

import (
	"encoding/json"
	"fmt"
	"net/http"
	"time"

	"github.com/go-chi/chi"
	"github.com/go-chi/chi/middleware"
	"github.com/rs/cors"
)

func srv(port string, isDev bool) *http.Server {
	r := initChi(isDev)

	if !isDev {
		r.Get("/", func(w http.ResponseWriter, r *http.Request) {
			// serve packr2 box here
			w.WriteHeader(http.StatusOK)
			json.NewEncoder(w).Encode("INDEX")
		})
		// r.Group(func(r chi.Router) {
		// 	r.Get("/", frontend.IndexRoute)
		// 	r.Get("/static/*", frontend.StaticRoute)
		// 	r.Get("/manifest.json", frontend.ManifestRoute)
		// 	r.Get("/favicon.ico", frontend.FaviconRoute)
		// })
	}

	r.Group(func(r chi.Router) {
		r.Get("/check", checkWallet)
		r.Get("/create", createWallet)
		r.Get("/balance", getBalance)
	})

	server := &http.Server{Addr: ":" + port, Handler: r}
	go func() {
		fmt.Println("Listening on port " + port)
		if err := server.ListenAndServe(); err != nil {
			fmt.Println("Server startup error:", err.Error())
		}
	}()
	return server
}

func initChi(isDev bool) *chi.Mux {
	r := chi.NewRouter()
	r.Use(middleware.RequestID)
	r.Use(middleware.Logger)
	r.Use(middleware.Recoverer)
	r.Use(middleware.Timeout(60 * time.Second))
	if isDev { // dev comes from port 8000
		cors := cors.New(cors.Options{
			AllowedOrigins:   []string{"*"},
			AllowedMethods:   []string{"GET", "POST", "PUT", "DELETE", "OPTIONS"},
			AllowedHeaders:   []string{"Accept", "Authorization", "Content-Type", "X-CSRF-Token", "X-User", "authorization"},
			AllowCredentials: true,
			MaxAge:           300, // Maximum value not ignored by any of major browsers
			//Debug:            true,
		})
		r.Use(cors.Handler)
	}
	return r
}