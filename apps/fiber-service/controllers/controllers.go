package controllers

import (
	"strconv"

	"github.com/gofiber/fiber/v2"
	"myapp.monorepo.fiber/helpers"
)

type User struct {
	Id int32 `json:"id"`
	Name string `json:"name"`
	Age int32 `json:"age"`
	Email string `json:"email"`
}

func GetAllUsers(c *fiber.Ctx) error {
	users := []User{
		{Id: 1, Name: "Alucard", Age: 22, Email: "alucard_gg@gmail.com"},
		{Id: 2, Name: "Brody", Age: 25, Email: "ireng_brody@gmail.com"},
		{Id: 3, Name: "Zilong", Age: 26, Email: "heroisneverfade@gmail.com"},
	}
	return helpers.Success(c, "Retrive all users", users)
}

func GetUser(c *fiber.Ctx) error {
	rawId := c.Params("id")
	id, err := strconv.ParseInt(rawId, 10, 64)
	if err != nil || id < 1 {
		return helpers.Error(c, 400, "Invalid Id", err)	
	}
	user := User{
		Id: int32(id),
		Name: "Skyes",
		Age: 24,
		Email: "skyes@email.com",
	}
	return helpers.Success(c, "Get user with id " + rawId, user)
}
