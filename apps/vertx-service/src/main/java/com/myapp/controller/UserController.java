package com.myapp.controller;

import com.myapp.helper.BaseResponse;
// import io.vertx.core.json.JsonArray;
import io.vertx.ext.web.RoutingContext;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class UserController {

    public static void Index(RoutingContext context) {
        BaseResponse.sendResponse(context, "Welcome to Vertx Service", "success", null);
    }

    // Get all users
    public static void getUsers(RoutingContext context) {
        // Dummy response for all users
        List<Map<String, Object>> users = new ArrayList<>();
        users.add(Map.of("id", 1, "name", "John Doe"));
        users.add(Map.of("id", 2, "name", "Jane Doe"));

        BaseResponse.sendResponse(context, "Request successful", "success", users);
    }

    // Get user by ID
    public static void getUserById(RoutingContext context) {
        String userId = context.pathParam("id");

        if (userId == null || userId.isEmpty()) {
            BaseResponse.sendResponse(context, "User ID is required", "forbidden", null);
            return;
        }

        // Dummy response for user with given ID
        Map<String, Object> user = new HashMap<>();
        user.put("id", Integer.parseInt(userId));
        user.put("name", "User " + userId);

        BaseResponse.sendResponse(context, "Request successful", "success", user);
    }

    // Delete user by ID
    public static void deleteUserById(RoutingContext context) {
        String userId = context.pathParam("id");

        if (userId == null || userId.isEmpty()) {
            BaseResponse.sendResponse(context, "User ID is required", "forbidden", null);
            return;
        }

        // Dummy response for delete confirmation
        Map<String, String> response = Map.of("message", "User with ID " + userId + " deleted");

        BaseResponse.sendResponse(context, "User deleted successfully", "success", response);
    }
}
