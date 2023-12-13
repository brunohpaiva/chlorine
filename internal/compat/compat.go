package compat

import (
	"github.com/labstack/echo/v4"
)

type ApiCompat interface {
	Install(app *echo.Echo) error
}
