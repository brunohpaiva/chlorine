package server

import (
	"context"
	"log"
	"os"

	"github.com/brunohpaiva/chlorine/internal/compat"
	"github.com/jackc/pgx/v5"
	"github.com/jackc/pgx/v5/pgxpool"
	"github.com/labstack/echo/v4"
)

func CreateServer() *echo.Echo {
	app := echo.New()

	ctx := context.Background()

	poolConfig, err := pgxpool.ParseConfig(os.Getenv("POSTGRES_URL"))
	if err != nil {
		log.Fatalf("Error while parsing postgres connection URL: %v", err)
	}
	poolConfig.AfterConnect = func(ctx context.Context, c *pgx.Conn) error {
		log.Println("after connect")
		return nil
	}
	poolConfig.AfterRelease = func(c *pgx.Conn) bool {
		log.Println("after release")
		return true
	}

	dbpool, err := pgxpool.NewWithConfig(ctx, poolConfig)
	if err != nil {
		log.Fatalf("Unable to create connection pool: %v", err)
	}
	// TODO: close the connection pool
	// defer dbpool.Close()

	malojaCompat := compat.NewMalojaApiCompat(os.Getenv("MALOJA_COMPAT_APIKEY"), dbpool)
	malojaCompat.Install(app)

	return app
}
