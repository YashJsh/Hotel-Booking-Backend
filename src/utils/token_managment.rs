use serde::{Serialize, Deserialize};
use std::{env, time::{SystemTime, UNIX_EPOCH}};
use jsonwebtoken::{EncodingKey, Header, decode, encode, Validation, Algorithm, DecodingKey};

use crate::models::user;

#[derive(Serialize, Deserialize)]
pub struct Claims{
    pub sub : Data,
    pub exp : usize
}

#[derive(Serialize, Deserialize)]
pub struct Data{
    pub id : String,
    pub role : String
}

pub fn create_token(user_id : &str, role : &str)-> String{
    let key = env::var("JWT_SECRET").expect("JWT secret missing");
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let data = Data{
        id : user_id.to_string(),
        role : role.to_string()
    };

    let claims = Claims{
        sub : data,
        exp : (now + 3600) as usize
    };

    let token = encode(&Header::default(), &claims,  &EncodingKey::from_secret(key.as_ref())).unwrap();

    token
}

pub fn verify_token(token : &str)-> bool{
    let decoding = decode::<Claims>(token, &DecodingKey::from_secret("secret".as_ref()),&Validation::new(Algorithm::HS256));
    match decoding{
        Ok(_)=> true,
        Err(_)=> false
    }
}

pub fn decode_token(token: &str) -> Option<Claims> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::new(Algorithm::HS256),
    )
    .ok()
    .map(|data| data.claims)
}