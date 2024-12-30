package routes

import (
	"github.com/gofiber/fiber/v2"
	"github.com/gofiber/fiber/v2/middleware/logger"
	"myapp.monorepo.fiber/controllers"
)

func SetupRoutes(app *fiber.App) {
	// Middleware
	api := app.Group("/api/user", logger.New())

	// routes
	api.Get("/", controllers.GetAllUsers)
	api.Get("/:id", controllers.GetUser)
}
