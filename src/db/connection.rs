use sqlx::{
    PgPool,
    postgres::PgPoolOptions
};
use std::env;

pub async fn get_pool()-> PgPool{
    let database_url = env::var("DATABASE_URI").expect("Database url is not given");

    let pool = PgPoolOptions::new()
    .max_connections(10)
    .connect(&database_url)
    .await
    .expect("Failed to create pool");

    pool
}