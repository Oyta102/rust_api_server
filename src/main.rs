use std::env;
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
use dotenv::dotenv;
use env_logger::Env;
use sqlx::MySqlPool;
use crate::middlewares::auth::JwtAuth;

mod controllers;
mod models;
mod routers;
mod utils;
mod handlers;
mod middlewares;

#[tokio::main]
async fn main()->std::io::Result<()> {
    env::set_var("RUST_LOG","info");
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    dotenv().ok();

    let add_host = env::var("ADD_HOST").expect("ADD_HOST must be set");
    let add_port = env::var("ADD_PORT").expect("ADD_PORT must be set");
    let addr = format!("{}:{}",add_host,add_port);

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = MySqlPool::connect(&db_url).await.expect("Failed to create pool");

    let app = HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(JwtAuth)
            .configure(routers::api::init)
            .app_data(web::Data::new(pool.clone()))
    })
        .bind(addr.as_str())?;
    println!("\n\n ===>服务启动,监听地址为：{} <=== \n\n",addr);
    app.run().await
}
