use actix_web::{HttpResponse, Responder, post, web};

use crate::models::user::{CreateUser, Role, SignInUser, SignUpResponse};
use crate::utils::{api_response::APIResponse, password::hash_password};

use sqlx::PgPool;

#[post("/auth/signup")]
pub async fn signup(data: web::Json<CreateUser>, pool: web::Data<PgPool>) -> impl Responder {
    let user = data.0;

    //Check if user already exists with that email
    let check_user = sqlx::query!("SELECT 1 AS exists FROM users WHERE email = $1", user.email)
        .fetch_optional(pool.get_ref())
        .await;

    match check_user {
        Ok(Some(_)) => {
            return HttpResponse::BadRequest()
                .json(APIResponse::<()>::error("EMAIL_ALREADY_EXISTS"));
        }

        Err(e) => {
            eprintln!("DB error: {:?}", e);
            return HttpResponse::InternalServerError()
                .json(APIResponse::<()>::error("DATABASE_ERROR"));
        }
        _ => {}
    };

    let hashed_password = match hash_password(&user.password) {
        Ok(hash) => hash,
        Err(_) => return HttpResponse::InternalServerError().body("Password hashing failed"),
    };

    let role = match user.role {
        Some(role) => role,
        None => Role::Customer,
    };

    let create_user = sqlx::query!(
        "INSERT into users (email, password, name, role, phone) VALUES ($1, $2, $3, $4, $5) RETURNING id",
        user.email,
        hashed_password,
        user.name,
        role.as_str(),
        user.phone
    )
    .fetch_one(pool.get_ref())
    .await;

    match create_user {
        Ok(record) => HttpResponse::Ok().json(APIResponse::success(SignUpResponse {
            id: record.id,
            email: user.email,
            name: user.name,
            role: role.as_str().to_string(),
            phone: user.phone.unwrap_or_default(),
        })),
        Err(e) => {
            eprintln!("Insert error: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/auth/signin")]
pub async fn signin(data: web::Json<SignInUser>, pool: web::Data<PgPool>) -> impl Responder {
    let body = data.0;
    let check_user = sqlx::query!("SELECT id FROM users WHERE email = $1", body.email)
        .fetch_optional(pool.get_ref())
        .await;

    let _user = match check_user {
        Ok(Some(user)) => user,
        Ok(None) => {
            return HttpResponse::Ok().json(APIResponse::<()>::error("USER_NOT_FOUND"));
        }
        Err(_) => {
            return HttpResponse::InternalServerError()
                .json(APIResponse::<()>::error("DATABASE_ERROR"));
        }
    };
    return HttpResponse::Ok().json(APIResponse::<()>::error("USER_NOT_FOUND"));
}
