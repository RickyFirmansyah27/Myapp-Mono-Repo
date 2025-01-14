use actix_web::{Responder};
use serde_json::json;
use crate::helper::base_response::BaseResponse;
use crate::helper::logger;  // Import the logger module

pub async fn get_users() -> impl Responder {
    // Log the start of the request
    logger::info("Starting to fetch users").await;

    let users = vec![
        json!({ "id": 1, "name": "Alice" }),
        json!({ "id": 2, "name": "Bob" })
    ];

    logger::info(format!("Successfully retrieved {} users", users.len())).await;
    
    BaseResponse::send_response("success", "Users retrieved successfully", Some(json!(users)))
}