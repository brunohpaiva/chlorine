package compat

import (
	"net/http"

	"github.com/labstack/echo/v4"
)

type malojaCompat struct {
	key string
}

func NewMalojaApiCompat(key string) ApiCompat {
	return &malojaCompat{
		key: key,
	}
}

type MalojaNewscrobbleRequestBody struct {
	Title    string   `json:"title"`
	Album    string   `json:"album"`
	Artists  []string `json:"artists"`
	Length   float32  `json:"length"`
	Duration float32  `json:"duration"`
	Key      string   `json:"key"`
}

func (m *malojaCompat) Install(app *echo.Echo) error {
	app.GET("/apis/mlj_1/serverinfo", func(c echo.Context) error {
		return c.JSON(http.StatusOK, map[string]interface{}{
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

	app.GET("/apis/mlj_1/test", func(c echo.Context) error {
		key := c.QueryParam("key")

		if key != "" && !m.isValidKey(key) {
			return c.JSON(http.StatusForbidden, map[string]interface{}{
				"status": "error",
				"error":  "Wrong API key",
			})
		} else {
			return c.JSON(http.StatusOK, map[string]interface{}{
				"status": "ok",
			})
		}
	})

	app.GET("/apis/mlj_1/scrobbles", func(c echo.Context) error {
		return c.JSON(http.StatusOK, []interface{}{})
	})

	app.POST("/apis/mlj_1/newscrobble", func(c echo.Context) error {
		reqBody := new(MalojaNewscrobbleRequestBody)

		if err := c.Bind(reqBody); err != nil {
			return err
		}

		if !m.isValidKey(reqBody.Key) {
			return c.JSON(http.StatusForbidden, m.invalidAuthResponse())
		}

		// TODO
		println("Received maloja scrobble", reqBody.Title)

		return c.JSON(http.StatusOK, map[string]interface{}{
			"status": "success",
		})
	})

	return nil
}

func (m *malojaCompat) isValidKey(suppliedKey string) bool {
	return suppliedKey == m.key
}

func (m *malojaCompat) invalidAuthResponse() map[string]interface{} {
	return map[string]interface{}{
		"status": "failure",
		"error": map[string]interface{}{
			"type": "authentication_fail",
			"desc": "Invalid or missing authentication",
		},
	}
}
