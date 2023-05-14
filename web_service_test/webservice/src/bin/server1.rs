use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::io;

// router
pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

// handler
pub async fn health_check_handler() -> impl Responder{  // TOOD 为什么返回的是Responder类型，而不是像HttpResponse之类的？
                                                        // 其实它返回的是Response，Responder默认实现了Into(Response)，即最终是会将Responder转换为Response，Response是Actix会自动将其转换为相应的HTTP响应
    HttpResponse::Ok().json("Actix Web Service is running")
}

// main, server
#[actix_rt::main]
async fn main() -> io::Result<()> {
    // 配置/初始化
    let app = move || App::new().configure(general_routes); // TODO 这里怎么理解？

    // 启动
    HttpServer::new(app).bind("localhost:5000")?.run().await
}