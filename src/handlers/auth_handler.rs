
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use crate::models::user::CreateUser;

#[get("/auth/signup")]
pub async fn signup(data : web::Json<CreateUser>)-> impl Responder{
    let user = data.0;
    println!("{:?}", user);
    
    HttpResponse::Ok()
}