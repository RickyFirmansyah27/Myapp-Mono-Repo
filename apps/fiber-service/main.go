package main

import (
	"github.com/gofiber/fiber/v2"
	"myapp.monorepo.fiber/routes"
)

func main() {
	app := fiber.New()

	app.Get("/", func(c *fiber.Ctx) error {
		return c.SendString("Hello, World!")
	})

	routes.SetupRoutes(app)
	
	app.Listen(":3000")
}
