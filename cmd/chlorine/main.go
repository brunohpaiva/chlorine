package main

import (
	"github.com/brunohpaiva/chlorine/internal/server"
)

func main() {
	app := server.CreateServer()

	app.Listen(":8000")
}
