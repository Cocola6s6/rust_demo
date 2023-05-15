use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::io;
use std::env;
use std::sync::Mutex;
use sqlx::postgres::PgPoolOptions;

#[path = "../handlers/mod.rs"]
mod handlers;

#[path = "../routers/routers.rs"]
mod routers;

#[path = "../models/mod.rs"]
mod models;

#[path = "../dbaccess/mod.rs"]
mod dbaccess;

#[path = "../common/errors.rs"]
mod errors;

#[path = "../common/state.rs"]
mod state;

use routers::*;
use state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();  //TODO 这里什么意思？
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is miss");
    let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

    // 配置/初始化
    let shared_data = web::Data::new(AppState {
        health_check_response: "I am OK.".to_string(),
        visit_count: Mutex::new(0),
        // courses: Mutex::new(vec![]),
        db: db_pool,
    });
    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(general_routes)
            .configure(course_routes)
    };

    // 启动
    HttpServer::new(app).bind("localhost:5000")?.run().await
}
