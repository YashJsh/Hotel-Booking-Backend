use actix_web::{self, HttpServer, App};

mod db;
mod handlers;
mod models;
mod utils;

use handlers::auth_handler::signup;


#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(
        || {
            App::new()
                .service(signup)
        }
    )
    .bind(("127.0.0.1", 8080))?
    .run()
    .await;

    Ok(())
}
