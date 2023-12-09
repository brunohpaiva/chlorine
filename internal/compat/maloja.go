package compat

import "github.com/gofiber/fiber/v2"

type MalojaCompat struct {
}

type MalojaNewscrobbleRequestBody struct {
	Title    string   `json:"title"`
	Album    string   `json:"album"`
	Artists  []string `json:"artists"`
	Length   float32  `json:"length"`
	Duration float32  `json:"duration"`
	Key      string   `json:"key"`
}

func (m MalojaCompat) Install(app *fiber.App) error {
	app.Get("/apis/mlj_1/serverinfo", func(c *fiber.Ctx) error {
		return c.JSON(map[string]interface{}{
			"name":          "chlorine (maloja compat)",
			"version":       []string{"3", "2", "1"},
			"versionstring": "3.2.1",
			"db_status": map[string]interface{}{
				"healthy":           true,
				"rebuildinprogress": false,
				"complete":          true,
			},
		})
	})

	app.Get("/apis/mlj_1/test", func(c *fiber.Ctx) error {
		return c.JSON(map[string]interface{}{
			"status": "ok",
		})
	})

	app.Get("/apis/mlj_1/scrobbles", func(c *fiber.Ctx) error {
		return c.JSON([]interface{}{})
	})

	app.Post("/apis/mlj_1/newscrobble", func(c *fiber.Ctx) error {
		reqBody := new(MalojaNewscrobbleRequestBody)

		if err := c.BodyParser(reqBody); err != nil {
			return err
		}

		// TODO
		println("Received maloja scrobble", reqBody.Title)

		return c.JSON(map[string]interface{}{
			"status": "success",
		})
	})

	return nil
}
