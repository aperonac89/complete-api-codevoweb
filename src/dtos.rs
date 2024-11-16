use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::models::User;

#[derive(Validate, Debug, Default, Deserialize, Serialize, Clone)]
pub struct RegisterUSerDto {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,

    #[validate(
        length(min = 1, message = "email is required"),
        email(message = "email is invalid")
    )]
    pub email: String,

    #[validate(
        length(min = 1, message = "password is required"),
        length(min = 6, message = "password must be at least 6 characters")
    )]
    pub password: String,

    #[validate(
        length(min = 1, message = "please confirm your password"),
        must_match(other = "password", message = "passwords do not match")
    )]
    pub password_confirmation: String
} 

#[derive(Serialize, Deserialize, Validate)]
pub struct LoginUserDto {
    #[validate(
        length(min = 1, message = "email is required"),
        email(message = "email is invalid")
    )]
    pub email: String,

    #[validate(
        length(min = 1, message = "password is required"),
        length(min = 6, message = "password must be at least 6 characters")
    )]
    pub password: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct RequestQueryDto {
    #[validate(range(min=1))]
    pub page: Option<usize>,

    #[validate(range(min=1, max=50))]
    pub limit: Option<usize>,
}

#[derive(Serialize, Deserialize)]
pub struct FilterUserDto {
    pub id: String, 
    pub name: String, 
    pub email: String,
    pub role: String, 
    pub photo: String, 
    pub verified: bool,
    #[serde(rename="created_at")]
    pub created_at: DateTime<Utc>,
    #[serde(rename="updated_at")]
    pub updated_at: DateTime<Utc>,
}

impl FilterUserDto {
    pub fn filter_user(user: &User) -> Self {
        FilterUserDto {
            id: user.id.to_string(),
            email: user.email.to_string(),
            name: user.name.to_string(),
            photo: user.photo.to_string(),
            role: user.role.to_string(),
            verified: user.verified,
            created_at: user.created_at,
            updated_at: user.updated_at
        }
    }

    pub fn filter_users(users: &[Users]) -> Vec<FilterUserDto> {
        users.iter().map(FilterUserDto::filter_user).collect()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserData {
    pub user: FilterUserDto,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserResponseDto {
    pub status: String,
    pub data: UserData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserListResponseDto {
    pub status: String,
    pub data: Vec<FilterUserDto>,
    pub results: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserLoginResponseDto {
    pub status: String,
    pub token: String,
}