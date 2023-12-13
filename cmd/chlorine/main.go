package main

import (
	"os"

	"github.com/brunohpaiva/chlorine/internal/server"
	"github.com/joho/godotenv"
)

func main() {
	loadEnv()

	app := server.CreateServer()

	app.Start(":8000")
}

func loadEnv() {
	env := os.Getenv("CHLORINE_ENV")
	if env == "" {
		env = "development"
	}

	godotenv.Load(".env." + env + ".local")
	if env != "test" {
		godotenv.Load(".env.local")
	}
	godotenv.Load(".env." + env)
	godotenv.Load(".env")
}
