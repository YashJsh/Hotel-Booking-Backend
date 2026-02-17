use actix_web::{self, HttpServer, App};

mod db;
mod handlers;

use handlers::auth_handler::hello;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(
        || {
            App::new()
                .service(hello)
        }
    )
    .bind(("127.0.0.1", 8080))?
    .run()
    .await;

    Ok(())
}
