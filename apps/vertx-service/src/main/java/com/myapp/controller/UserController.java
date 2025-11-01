package com.myapp.controller;

import com.myapp.helper.BaseResponse;
import com.myapp.service.UserService;
import io.vertx.ext.web.RoutingContext;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.List;
import java.util.Map;
import java.util.Optional;

public class UserController {

    private static final Logger logger = LoggerFactory.getLogger(UserController.class);

    public static void Index(RoutingContext context) {
        logger.info("Handling Index request");
        BaseResponse.sendResponse(context, "Welcome to Vertx Service", "success", null);
    }

    // Get all users
    public static void getUsers(RoutingContext context) {
        logger.info("User Controller | getUsers");

        try {
            List<Map<String, Object>> users = UserService.getAllUsers();
            logger.info("Returning {} users", users.size());
            BaseResponse.sendResponse(context, "Request successful", "success", users);
        } catch (Exception e) {
            logger.error("Error getting users", e);
            BaseResponse.sendResponse(context, "Internal server error", "error", null);
        }
    }

    // Get user by ID
    public static void getUserById(RoutingContext context) {
        String userId = context.pathParam("id");
        logger.info("User Controller | getUserById for ID: {}", userId);

        if (userId == null || userId.isEmpty()) {
            logger.warn("User ID is required, returning error");
            BaseResponse.sendResponse(context, "User ID is required", "forbidden", null);
            return;
        }

        try {
            Optional<Map<String, Object>> user = UserService.getUserById(userId);
            if (user.isPresent()) {
                logger.info("User found: {}", user.get());
                BaseResponse.sendResponse(context, "Request successful", "success", user.get());
            } else {
                logger.warn("User not found with ID: {}", userId);
                BaseResponse.sendResponse(context, "User not found", "not_found", null);
            }
        } catch (Exception e) {
            logger.error("Error getting user by ID: {}", userId, e);
            BaseResponse.sendResponse(context, "Internal server error", "error", null);
        }
    }

    // Create new user
    public static void createUser(RoutingContext context) {
        logger.info("User Controller | createUser");

        try {
            // Get user data from request body
            Map<String, Object> userData =  context.body().asJsonObject().getMap();
            
            if (userData == null || userData.isEmpty()) {
                logger.warn("User data is required");
                BaseResponse.sendResponse(context, "User data is required", "bad_request", null);
                return;
            }

            Map<String, Object> newUser = UserService.createUser(userData);
            logger.info("User created successfully: {}", newUser);
            BaseResponse.sendResponse(context, "User created successfully", "success", newUser);
            
        } catch (Exception e) {
            logger.error("Error creating user", e);
            BaseResponse.sendResponse(context, "Internal server error", "error", null);
        }
    }

    // Update user by ID
    public static void updateUser(RoutingContext context) {
        String userId = context.pathParam("id");
        logger.info("User Controller | updateUser for ID: {}", userId);

        if (userId == null || userId.isEmpty()) {
            logger.warn("User ID is required, returning error");
            BaseResponse.sendResponse(context, "User ID is required", "forbidden", null);
            return;
        }

        try {
            // Get user data from request body
            Map<String, Object> userData = context.body().asJsonObject().getMap();
            
            if (userData == null || userData.isEmpty()) {
                logger.warn("User data is required");
                BaseResponse.sendResponse(context, "User data is required", "bad_request", null);
                return;
            }

            Optional<Map<String, Object>> updatedUser = UserService.updateUser(userId, userData);
            if (updatedUser.isPresent()) {
                logger.info("User updated successfully: {}", updatedUser.get());
                BaseResponse.sendResponse(context, "User updated successfully", "success", updatedUser.get());
            } else {
                logger.warn("Failed to update user with ID: {}", userId);
                BaseResponse.sendResponse(context, "Failed to update user", "error", null);
            }
        } catch (Exception e) {
            logger.error("Error updating user by ID: {}", userId, e);
            BaseResponse.sendResponse(context, "Internal server error", "error", null);
        }
    }

    // Delete user by ID
    public static void deleteUserById(RoutingContext context) {
        String userId = context.pathParam("id");
        logger.info("User Controller | deleteUserById for ID: {}", userId);

        if (userId == null || userId.isEmpty()) {
            logger.warn("User ID is required, returning error");
            BaseResponse.sendResponse(context, "User ID is required", "forbidden", null);
            return;
        }

        try {
            Optional<Map<String, String>> result = UserService.deleteUserById(userId);
            if (result.isPresent()) {
                logger.info("User deleted successfully: {}", result.get());
                BaseResponse.sendResponse(context, "User deleted successfully", "success", result.get());
            } else {
                logger.warn("Failed to delete user with ID: {}", userId);
                BaseResponse.sendResponse(context, "Failed to delete user", "error", null);
            }
        } catch (Exception e) {
            logger.error("Error deleting user by ID: {}", userId, e);
            BaseResponse.sendResponse(context, "Internal server error", "error", null);
        }
    }
}
