use actix_cors::Cors;
use actix_web::{http, web, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::io;
use std::sync::Mutex;

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
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("没找到配置文件"); // 文件路径是webservice路径开始的
    let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

    // 配置/初始化
    let shared_data = web::Data::new(AppState {
        health_check_response: "I am OK.".to_string(),
        visit_count: Mutex::new(0),
        // courses: Mutex::new(vec![]),
        db: db_pool,
    });

    // // TOOD 本来想把跨域配置写在闭包之外的，但是它告诉我App需要实现的是Fn trait，
    // // 跨域配置
    // let cors = Cors::default()
    //     .allowed_origin("http://localhost:8080/")
    //     .allowed_origin_fn(|origin, _req_head| origin.as_bytes().starts_with(b"http://localhost"))
    //     .allowed_methods(vec!["GET", "POST"])
    //     .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
    //     .allowed_header(http::header::CONTENT_TYPE)
    //     .max_age(3600);

    let app = move || {
        // 跨域配置
        let cors = Cors::default()
            .allowed_origin("http://localhost:8080/")
            .allowed_origin_fn(|origin, _req_head| {
                origin.as_bytes().starts_with(b"http://localhost")
            })
            .allowed_methods(vec!["GET", "POST", "DELETE", "UPDATE"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .app_data(shared_data.clone())
            .wrap(cors)
            .configure(general_routes)
            .configure(course_routes)
    };

    // 启动
    HttpServer::new(app).bind("localhost:5000")?.run().await
}
