use serde_json::json;
use crate::helper::logger;

pub struct UserService;

impl UserService {
    pub fn new() -> Self {
        UserService
    }

    pub async fn get_users(&self) -> Result<Vec<serde_json::Value>, String> {
        logger::info("Starting to fetch users from service layer").await;

        let users = vec![
            json!({ "id": 1, "name": "Alice", "email": "alice@example.com" }),
            json!({ "id": 2, "name": "Bob", "email": "bob@example.com" }),
            json!({ "id": 3, "name": "Charlie", "email": "charlie@example.com" })
        ];

        logger::info(format!("Successfully retrieved {} users from service layer", users.len())).await;
        
        Ok(users)
    }

    pub async fn get_user_by_id(&self, user_id: u32) -> Result<Option<serde_json::Value>, String> {
        logger::info(format!("Fetching user with ID: {}", user_id)).await;

        let users = self.get_users().await?;
        let user = users.into_iter().find(|u| u["id"].as_u64() == Some(user_id as u64));

        match user {
            Some(found_user) => {
                logger::info(format!("Successfully found user with ID: {}", user_id)).await;
                Ok(Some(found_user))
            }
            None => {
                logger::warn(format!("User with ID {} not found", user_id)).await;
                Ok(None)
            }
        }
    }

    pub async fn create_user(&self, user_data: serde_json::Value) -> Result<serde_json::Value, String> {
        logger::info("Creating new user in service layer").await;

        if !user_data["name"].is_string() || !user_data["email"].is_string() {
            return Err("Invalid user data: name and email are required".to_string());
        }

        let new_user = json!({
            "id": 4,
            "name": user_data["name"],
            "email": user_data["email"]
        });

        logger::info("Successfully created new user in service layer").await;
        Ok(new_user)
    }

    pub async fn update_user(&self, user_id: u32, user_data: serde_json::Value) -> Result<Option<serde_json::Value>, String> {
        logger::info(format!("Updating user with ID: {}", user_id)).await;

        let existing_user = self.get_user_by_id(user_id).await?;
        if existing_user.is_none() {
            return Ok(None);
        }

        let updated_user = json!({
            "id": user_id,
            "name": user_data["name"].as_str().unwrap_or("Unknown"),
            "email": user_data["email"].as_str().unwrap_or("unknown@example.com"),
            "updated_at": chrono::Utc::now().to_rfc3339()
        });

        logger::info(format!("Successfully updated user with ID: {}", user_id)).await;
        Ok(Some(updated_user))
    }

    pub async fn delete_user(&self, user_id: u32) -> Result<bool, String> {
        logger::info(format!("Deleting user with ID: {}", user_id)).await;

        let existing_user = self.get_user_by_id(user_id).await?;
        if existing_user.is_none() {
            logger::warn(format!("Cannot delete user with ID {}: not found", user_id)).await;
            return Ok(false);
        }

        logger::info(format!("Successfully deleted user with ID: {}", user_id)).await;
        Ok(true)
    }
}