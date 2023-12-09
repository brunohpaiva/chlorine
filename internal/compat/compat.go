package compat

import "github.com/gofiber/fiber/v2"

type ApiCompat interface {
	Install(app *fiber.App) error
}
