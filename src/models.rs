use chrono::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy, sqlx::Type, PartialEq)]
#[sqlx(type_name="user_role", rename_all="lowercase")]
pub enum UserRole{
    Admin,
    Moderator,
    User,
}

impl UserRole {
    pub fn to_str(&self) -> &str {
        match self {
            UserRole::Admin => "admin",
            UserRole::Moderator => "moderator",
            UserRole::User => "user" 
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::Type, sqlx::FromRow)]
pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: UserRole,
    pub photo: String,
    pub verified: bool,
    #[serde(rename="createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename="updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}