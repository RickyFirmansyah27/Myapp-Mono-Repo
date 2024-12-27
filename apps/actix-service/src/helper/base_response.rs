use actix_web::{HttpResponse, Responder};
use serde_json::json;

pub struct BaseResponse;

impl BaseResponse {
    pub fn send_response(
        response_type: &str,
        res_message: &str,
        result: Option<serde_json::Value>,
    ) -> impl Responder {
        match response_type {
            "created" => Self::created_response(res_message),
            "success" => Self::success_response(result, res_message),
            "unauthorized" | "forbidden" => Self::unauthorized_response(res_message),
            "internalServerError" => Self::internal_server_error_response(),
            _ => Self::success_response(result, res_message),
        }
    }

    fn created_response(message: &str) -> HttpResponse {
        HttpResponse::Created()
            .content_type("application/json")
            .json(json!({
                "status": "created",
                "message": message
            }))
    }

    fn success_response(result: Option<serde_json::Value>, message: &str) -> HttpResponse {
        HttpResponse::Ok()
            .content_type("application/json")
            .json(json!({
                "status": "success",
                "message": message,
                "data": result.unwrap_or(json!(null))
            }))
    }

    fn unauthorized_response(message: &str) -> HttpResponse {
        HttpResponse::Forbidden()
            .content_type("application/json")
            .json(json!({
                "status": "unauthorized",
                "message": message
            }))
    }

    fn internal_server_error_response() -> HttpResponse {
        HttpResponse::InternalServerError()
            .content_type("application/json")
            .json(json!({
                "status": "error",
                "message": "Internal Server Error"
            }))
    }
}
