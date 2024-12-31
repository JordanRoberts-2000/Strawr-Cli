package main

import (
	"fmt"
	"net/http"
	"strconv"
)

func main() {
	router := http.NewServeMux()

	router.HandleFunc("GET /api/todos", func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
	w.Write([]byte("Hello world"))
	})
	var port int = 3000
	server := http.Server{
		Addr:    ":" + strconv.Itoa(port),
		Handler: router,
	}

	fmt.Println("Server stared on port: " + strconv.Itoa(port))
	if err := server.ListenAndServe(); err != nil {
		fmt.Println(err.Error())
	}
}