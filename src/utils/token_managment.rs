use serde::{Serialize, Deserialize};
use std::{env, time::{SystemTime, UNIX_EPOCH}};
use jsonwebtoken::{EncodingKey, Header, decode, encode, Validation, Algorithm, DecodingKey};

#[derive(Serialize, Deserialize)]
pub struct Claims{
    sub : String,
    exp : usize
}


pub fn create_token(user_id : &str)-> String{
    let key = env::var("JWT_SECRET").expect("JWT secret missing");
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let claims = Claims{
        sub : user_id.to_string(),
        exp : (now + 3600) as usize
    };

    let token = encode(&Header::default(), &claims,  &EncodingKey::from_secret(key.as_ref())).unwrap();

    token
}

pub fn decode_token(token : &str)-> bool{
    let decoding = decode::<Claims>(token, &DecodingKey::from_secret("secret".as_ref()),&Validation::new(Algorithm::HS256));
    match decoding{
        Ok(_)=> true,
        Err(_)=> false
    }
}