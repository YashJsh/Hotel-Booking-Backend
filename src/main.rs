use actix_web::{self, App, HttpServer, web};

mod db;
mod handlers;
mod models;
mod utils;
mod middleware;

use handlers::auth_handler::signup;

use crate::handlers::auth_handler::signin;


#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let pool = db::connection::get_pool().await;
    HttpServer::new(
        move || {
            
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(signup)
                .service(signin)
                
        }
    )
    .bind(("127.0.0.1", 8080))?
    .run()
    .await;

    Ok(())
}
