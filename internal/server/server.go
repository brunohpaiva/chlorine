package server

import (
	"html/template"
	"io"
	"net/http"
	"os"

	"github.com/brunohpaiva/chlorine/internal/compat"
	"github.com/labstack/echo/v4"
)

type templateEngine struct {
	templates *template.Template
}

func (e *templateEngine) Render(w io.Writer, name string, data interface{}, c echo.Context) error {
	return e.templates.ExecuteTemplate(w, name, data)
}

func CreateServer() *echo.Echo {
	app := echo.New()

	app.Renderer = &templateEngine{
		templates: template.Must(template.ParseGlob("views/*.html")),
	}

	malojaCompat := compat.NewMalojaApiCompat(os.Getenv("MALOJA_COMPAT_APIKEY"))
	malojaCompat.Install(app)

	app.GET("/", func(c echo.Context) error {
		return c.Render(http.StatusOK, "index.html", map[string]interface{}{
			"Title": "Chlorine",
		})
	})

	return app
}
