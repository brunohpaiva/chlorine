package server

import (
	"github.com/brunohpaiva/chlorine/internal/compat"
	"github.com/gofiber/fiber/v2"
	"github.com/gofiber/template/html/v2"
)

func CreateServer() *fiber.App {
	engine := html.New("./views", ".html")

	app := fiber.New(fiber.Config{
		Views: engine,
	})

	malojaCompat := compat.MalojaCompat{}
	malojaCompat.Install(app)

	app.Get("/", func(c *fiber.Ctx) error {
		return c.Render("index", fiber.Map{
			"Title": "Chlorine",
		})
	})

	return app
}
