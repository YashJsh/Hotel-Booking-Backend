use actix_web::{HttpResponse, Responder, post, web};

use crate::models::hotel::CreateHotel;

use sqlx::PgPool;

#[post("/api/hotels")]
pub async fn signup(data: web::Json<CreateHotel>, pool: web::Data<PgPool>) -> impl Responder {
    let body = data.0;
    HttpResponse::Ok().body("Hello")
}