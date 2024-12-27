use actix_web::{Responder};
use serde_json::json;
use crate::helper::base_response::BaseResponse;

pub async fn get_users() -> impl Responder {
    let users = vec![
        json!({ "id": 1, "name": "Alice" }),
        json!({ "id": 2, "name": "Bob" })
    ];

    BaseResponse::send_response("success", "Users retrieved successfully", Some(json!(users)))
}
