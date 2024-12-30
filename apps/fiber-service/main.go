package main

import (
	"github.com/gofiber/fiber/v2"
	"github.com/gofiber/fiber/v2/log"
	"myapp.monorepo.fiber/helpers"
	"myapp.monorepo.fiber/routes"
)

type Data struct {
	Message string `json:"message"`
}

func main() {
	app := fiber.New()

	app.Get("/", func(c *fiber.Ctx) error {
		res := Data{Message: "Fiber Service"}
		return helpers.Success(c, "Welcome to Fiber Service", res)
	})

	log.Info("api path is /api/user")

	routes.SetupRoutes(app)
	
	app.Listen(":8106")
}
