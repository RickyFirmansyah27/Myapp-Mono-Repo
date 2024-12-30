package helpers

import (
	"github.com/gofiber/fiber/v2"
)

type BaseResponse struct {
    Success bool        `json:"success"`
    Message string      `json:"message"`
    Data    interface{} `json:"data,omitempty"`
    Error   interface{} `json:"error,omitempty"`
}

func Success(c *fiber.Ctx, message string, data interface{}) error {
    return c.Status(fiber.StatusOK).JSON(BaseResponse{
        Success: true,
        Message: message,
        Data:    data,
    })
}

func Error(c *fiber.Ctx, statusCode int, message string, err interface{}) error {
    return c.Status(statusCode).JSON(BaseResponse{
        Success: false,
        Message: message,
        Error:   err,
    })
}

func ValidationError(c *fiber.Ctx, message string, err interface{}) error {
    return Error(c, fiber.StatusBadRequest, message, err)
}

func NotFound(c *fiber.Ctx, message string) error {
    return Error(c, fiber.StatusNotFound, message, nil)
}

func ServerError(c *fiber.Ctx, err interface{}) error {
    return Error(c, fiber.StatusInternalServerError, "Internal Server Error", err)
}

func Unauthorized(c *fiber.Ctx, message string) error {
    return Error(c, fiber.StatusUnauthorized, message, nil)
}

func Forbidden(c *fiber.Ctx, message string) error {
    return Error(c, fiber.StatusForbidden, message, nil)
}