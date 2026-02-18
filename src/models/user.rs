use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};
use sqlx::Type;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUser {
    pub name: String,
    #[validate(email)]
    pub email : String,
    pub password : String,
    pub role :  Option<Role>,
    pub phone : Option<String>
}

#[derive(Debug, Deserialize, Type)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum Role{
    Customer,
    Owner
}

impl Role {
    pub fn as_str(&self) -> &str {
        match self {
            Role::Owner => "owner",
            Role::Customer => "customer",
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct SignUpResponse{
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
    pub phone: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct SignInUser{
    #[validate(email)]
    pub email : String,
    pub password : String,
}