package com.myapp.service;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Optional;

public class UserService {
    
    private static final Logger logger = LoggerFactory.getLogger(UserService.class);
    
    // Get all users
    public static List<Map<String, Object>> getAllUsers() {
        logger.info("UserService | getAllUsers");
        
        // Dummy data for all users
        List<Map<String, Object>> users = new ArrayList<>();
        users.add(Map.of("id", 1, "name", "John Doe", "email", "john.doe@example.com"));
        users.add(Map.of("id", 2, "name", "Jane Doe", "email", "jane.doe@example.com"));
        users.add(Map.of("id", 3, "name", "Bob Smith", "email", "bob.smith@example.com"));
        
        logger.info("Returning {} users", users.size());
        return users;
    }
    
    // Get user by ID
    public static Optional<Map<String, Object>> getUserById(String userId) {
        logger.info("UserService | getUserById for ID: {}", userId);
        
        if (userId == null || userId.isEmpty()) {
            logger.warn("User ID is null or empty");
            return Optional.empty();
        }
        
        try {
            int id = Integer.parseInt(userId);
            
            // Dummy data - in real application this would come from database
            Map<String, Object> user = new HashMap<>();
            user.put("id", id);
            user.put("name", "User " + id);
            user.put("email", "user" + id + "@example.com");
            
            logger.info("User found: {}", user);
            return Optional.of(user);
            
        } catch (NumberFormatException e) {
            logger.error("Invalid user ID format: {}", userId, e);
            return Optional.empty();
        }
    }
    
    // Delete user by ID
    public static Optional<Map<String, String>> deleteUserById(String userId) {
        logger.info("UserService | deleteUserById for ID: {}", userId);
        
        if (userId == null || userId.isEmpty()) {
            logger.warn("User ID is null or empty");
            return Optional.empty();
        }
        
        try {
            int id = Integer.parseInt(userId);
            
            // Dummy delete operation - in real application this would delete from database
            // Here we just simulate successful deletion
            Map<String, String> response = new HashMap<>();
            response.put("message", "User with ID " + id + " deleted successfully");
            response.put("deletedUserId", userId);
            
            logger.info("User deleted successfully: {}", response);
            return Optional.of(response);
            
        } catch (NumberFormatException e) {
            logger.error("Invalid user ID format: {}", userId, e);
            return Optional.empty();
        }
    }
    
    // Create new user (bonus method for future use)
    public static Map<String, Object> createUser(Map<String, Object> userData) {
        logger.info("UserService | createUser with data: {}", userData);
        
        // Dummy user creation - in real application this would save to database
        Map<String, Object> newUser = new HashMap<>(userData);
        newUser.put("id", (int) (Math.random() * 1000)); // Generate random ID
        newUser.put("createdAt", System.currentTimeMillis());
        
        logger.info("User created successfully: {}", newUser);
        return newUser;
    }
    
    // Update user (bonus method for future use)
    public static Optional<Map<String, Object>> updateUser(String userId, Map<String, Object> userData) {
        logger.info("UserService | updateUser for ID: {} with data: {}", userId, userData);
        
        if (userId == null || userId.isEmpty()) {
            logger.warn("User ID is null or empty");
            return Optional.empty();
        }
        
        try {
            int id = Integer.parseInt(userId);
            
            // Dummy update operation - in real application this would update in database
            Map<String, Object> updatedUser = new HashMap<>(userData);
            updatedUser.put("id", id);
            updatedUser.put("updatedAt", System.currentTimeMillis());
            
            logger.info("User updated successfully: {}", updatedUser);
            return Optional.of(updatedUser);
            
        } catch (NumberFormatException e) {
            logger.error("Invalid user ID format: {}", userId, e);
            return Optional.empty();
        }
    }
}