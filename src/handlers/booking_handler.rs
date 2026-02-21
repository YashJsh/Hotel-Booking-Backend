use actix_web::{
    HttpMessage, HttpRequest, HttpResponse, Responder, post,
    web::{self},
};
use serde::{Deserialize, Serialize};

use crate::{
    middleware::auth_middleware::AuthUser,
    models::hotel::{CreateHotel, CreateRooms, CreateRoomResponse},
    utils::api_response::APIResponse,
};

use sqlx::{PgPool, prelude::FromRow, types::BigDecimal};

#[derive(FromRow, Serialize, Deserialize)]
pub struct Hotel {
    id: String,
    owner_id: String,
    name: String,
    description: Option<String>,
    city: String,
    country: String,
    amenities: Option<Vec<String>>, 
    rating: Option<BigDecimal>,     
    total_reviews: Option<i32>,
}

#[post("/api/hotels")]
pub async fn create_hotel(
    data: web::Json<CreateHotel>,
    pool: web::Data<PgPool>,
    req: HttpRequest,
) -> impl Responder {
    let body = data.into_inner();

    let extensions = req.extensions();
    let auth_user = match extensions.get::<AuthUser>() {
        Some(user) => user,
        None => return HttpResponse::Unauthorized().json(APIResponse::<()>::error("UNAUTHORIZED")),
    };

    if auth_user.role != "owner".to_string() {
        return HttpResponse::Forbidden().json(APIResponse::<()>::error("FORBIDDEN"));
    }

    let hotel = sqlx::query_as!(
        Hotel,
        r#"
        INSERT INTO hotels (owner_id, name, description, city, country, amenities)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING 
            id, 
            owner_id as "owner_id!", 
            name, 
            description, 
            city, 
            country, 
            amenities as "amenities!", 
            rating as "rating!", 
            total_reviews as "total_reviews!"
        "#,
        auth_user.id,
        body.name,
        body.description,
        body.city,
        body.country,
        &body.amenities
    )
    .fetch_one(pool.get_ref())
    .await;

    match hotel {
        Ok(data) => return HttpResponse::Ok().json(APIResponse::success(data)),
        Err(_) => {
            return HttpResponse::InternalServerError()
                .json(APIResponse::<()>::error("FAILED_TO_CREATE_HOTEL"));
        }
    }
}

#[derive(FromRow)]
struct HotelOwner {
    id: Option<String>,
    owner_id: Option<String>,
}

#[derive(FromRow)]
struct RoomExist{
    room_number : Option<String>
}

#[post("/api/hotels/:hotelId/rooms")]
pub async fn create_rooms(
    data: web::Json<CreateRooms>,
    pool: web::Data<PgPool>,
    path: web::Path<String>,
    req: HttpRequest,
) -> impl Responder {
    let hotel_id = path.into_inner();

    let body = data.into_inner();

    let extensions = req.extensions();
    let auth_user = match extensions.get::<AuthUser>() {
        Some(user) => user,
        None => return HttpResponse::Unauthorized().json(APIResponse::<()>::error("UNAUTHORIZED")),
    };

    //check if the role is owner;
    if auth_user.role == "customer".to_string() {
        return HttpResponse::Unauthorized().json(APIResponse::<()>::error("UNAUTHORIZED"));
    }
    
    let hotel = sqlx::query_as!(
        HotelOwner,
        "SELECT id, owner_id FROM hotels WHERE id = $1",
        hotel_id
    )
    .fetch_optional(pool.get_ref())
    .await;

    match hotel{
        Ok(Some(hotel))=> {
            if hotel.owner_id != Some(auth_user.id.clone()){
                return HttpResponse::Forbidden().json(APIResponse::<()>::error("FORBIDDEN"));
            }
        }
        Ok(None)=>{
            return HttpResponse::NotFound().json(APIResponse::<()>::error("NOTFOUND"));
        }
        Err(_)=>{
            return HttpResponse::InternalServerError().json(APIResponse::<()>::error("SERVER ERROR"))
        }
    }

    //check if room already exists;
    let room = sqlx::query_as!(
        RoomExist,
        "Select room_number from rooms where hotel_id = $1",
        hotel_id
    )
    .fetch_optional(pool.as_ref())
    .await;

    match room{
        Ok(Some(room))=> {
            if room.room_number.unwrap() == body.room_number{
                return HttpResponse::BadRequest().json(APIResponse::<()>::error("ROOM_ALREADY_EXISTS"));
            }
        }
        Ok(_)=> {}
        Err(_)=>{
            return HttpResponse::InternalServerError().json(APIResponse::<()>::error("ERROR IN FETCHING ROOM"))
        }
    }
  
    //create room
    let new_room = sqlx::query_as!(
        CreateRoomResponse,
        "INSERT INTO rooms (hotel_id, room_number, room_type, price_per_night, max_occupancy) Values($1, $2, $3, $4, $5)
        RETURNING 
        id,
        hotel_id,
        room_number,
        room_type,
        price_per_night,
        max_occupancy",
        hotel_id,
        body.room_number,
        body.room_type,
        body.price_per_night,
        body.max_occupancy
    )
    .fetch_one(pool.get_ref())
    .await;

    match new_room{
        Ok(data) => return HttpResponse::Ok().json(APIResponse::success(data)),
        Err(_)=> return HttpResponse::InternalServerError().json(APIResponse::<()>::error("Error in creating room"))
    }
}
