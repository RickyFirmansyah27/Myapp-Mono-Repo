package com.myapp.controller;

import com.myapp.helper.BaseResponse;
import io.vertx.ext.web.RoutingContext;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class UserController {

    private static final Logger logger = LoggerFactory.getLogger(UserController.class);

    public static void Index(RoutingContext context) {
        logger.info("Handling Index request");
        BaseResponse.sendResponse(context, "Welcome to Vertx Service", "success", null);
    }

    // Get all users
    public static void getUsers(RoutingContext context) {
        logger.info("User Controller | getUsers");

        // Dummy response for all users
        List<Map<String, Object>> users = new ArrayList<>();
        users.add(Map.of("id", 1, "name", "John Doe"));
        users.add(Map.of("id", 2, "name", "Jane Doe"));

        // Log the response
        logger.info("Returning users: {}", users);
        BaseResponse.sendResponse(context, "Request successful", "success", users);
    }

    // Get user by ID
    public static void getUserById(RoutingContext context) {
        String userId = context.pathParam("id");
        logger.info("Handling getUserById request for ID: {}", userId);

        if (userId == null || userId.isEmpty()) {
            logger.warn("User ID is required, returning error");
            BaseResponse.sendResponse(context, "User ID is required", "forbidden", null);
            return;
        }

        // Dummy response for user with given ID
        Map<String, Object> user = new HashMap<>();
        user.put("id", Integer.parseInt(userId));
        user.put("name", "User " + userId);

        // Log the response
        logger.info("User fetched successfully: {}", user);
        BaseResponse.sendResponse(context, "Request successful", "success", user);
    }

    // Delete user by ID
    public static void deleteUserById(RoutingContext context) {
        String userId = context.pathParam("id");
        logger.info("Handling deleteUserById request for ID: {}", userId);

        if (userId == null || userId.isEmpty()) {
            logger.warn("User ID is required, returning error");
            BaseResponse.sendResponse(context, "User ID is required", "forbidden", null);
            return;
        }

        // Dummy response for delete confirmation
        Map<String, String> response = Map.of("message", "User with ID " + userId + " deleted");

        // Log the response
        logger.info("User deleted successfully: {}", response);
        BaseResponse.sendResponse(context, "User deleted successfully", "success", response);
    }
}
