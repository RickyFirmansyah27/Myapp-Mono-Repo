package controllers

import "github.com/gofiber/fiber/v2"

func GetAllUsers(c *fiber.Ctx) error {
	return c.SendString("GET All Users")
}

func GetUser(c *fiber.Ctx) error {
	return c.SendString("GET User with ID " + c.Params("id"))
}
