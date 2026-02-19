use actix_web::{
    Error, HttpMessage, HttpResponse,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::{self, Next},
};

use crate::utils::token_managment::{decode_token, verify_token};

#[derive(Clone, Debug)]
pub struct AuthUser {
    pub id: String,
    pub role : String
}

async fn auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody + 'static>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    // pre-processing
    let auth_header = req.headers().get("Authorization");
    let token = match auth_header
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
    {
        Some(token) => token,
        None => {
            return Ok(req.into_response(
                HttpResponse::Unauthorized().body("Missing or invalid Authorization header"),
            ));
        }
    };

    if !verify_token(token) {
        return Ok(req.into_response(HttpResponse::Unauthorized().body("Invalid token")));
    }
    let claims = match decode_token(token) {
        Some(data) => data,
        None => {
            return Ok(req.into_response(HttpResponse::Unauthorized().body("Invalid token")));
        }
    };

    let auth_user = AuthUser { id: claims.sub.id, role : claims.sub.role };

    req.extensions_mut().insert(auth_user);
    let res = next.call(req).await?;

    Ok(res.map_into_boxed_body())
}
