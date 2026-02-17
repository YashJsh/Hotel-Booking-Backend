use serde::Deserialize;
use validator::{Validate, ValidationError};

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUser {
    pub name: String,
    #[validate(email)]
    pub email : String,
    pub password : String,
    pub role :  Option<Role>,
    pub phone : Option<String>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role{
    Customer,
    Owner
}