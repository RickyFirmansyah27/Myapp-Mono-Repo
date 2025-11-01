use actix_web::{Responder, web};
use serde_json::json;
use crate::helper::base_response::BaseResponse;
use crate::services::user_service::UserService;

/// Get all users
pub async fn get_users() -> impl Responder {
    let user_service = UserService::new();
    
    match user_service.get_users().await {
        Ok(users) => BaseResponse::send_response("success", "Users retrieved successfully", Some(json!(users))),
        Err(error) => BaseResponse::send_response("internalServerError", &format!("Failed to retrieve users: {}", error), None)
    }
}

/// Get a user by ID
pub async fn get_user_by_id(path: web::Path<u32>) -> impl Responder {
    let user_id = path.into_inner();
    let user_service = UserService::new();
    
    match user_service.get_user_by_id(user_id).await {
        Ok(Some(user)) => BaseResponse::send_response("success", "User retrieved successfully", Some(user)),
        Ok(None) => BaseResponse::send_response("unauthorized", "User not found", None),
        Err(error) => BaseResponse::send_response("internalServerError", &format!("Failed to retrieve user: {}", error), None)
    }
}

/// Create a new user
pub async fn create_user(user_data: web::Json<serde_json::Value>) -> impl Responder {
    let user_service = UserService::new();
    
    match user_service.create_user(user_data.into_inner()).await {
        Ok(user) => BaseResponse::send_response("created", "User created successfully", Some(user)),
        Err(error) => BaseResponse::send_response("internalServerError", &format!("Failed to create user: {}", error), None)
    }
}

/// Update an existing user
pub async fn update_user(path: web::Path<u32>, user_data: web::Json<serde_json::Value>) -> impl Responder {
    let user_id = path.into_inner();
    let user_service = UserService::new();
    
    match user_service.update_user(user_id, user_data.into_inner()).await {
        Ok(Some(user)) => BaseResponse::send_response("success", "User updated successfully", Some(user)),
        Ok(None) => BaseResponse::send_response("unauthorized", "User not found", None),
        Err(error) => BaseResponse::send_response("internalServerError", &format!("Failed to update user: {}", error), None)
    }
}

/// Delete a user
pub async fn delete_user(path: web::Path<u32>) -> impl Responder {
    let user_id = path.into_inner();
    let user_service = UserService::new();
    
    match user_service.delete_user(user_id).await {
        Ok(true) => BaseResponse::send_response("success", "User deleted successfully", None),
        Ok(false) => BaseResponse::send_response("unauthorized", "User not found", None),
        Err(error) => BaseResponse::send_response("internalServerError", &format!("Failed to delete user: {}", error), None)
    }
}