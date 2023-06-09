use crate::handlers::{course::*, general::*};
use actix_web::web;

// router1
// curl -X GET localhost:5000/health
pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("health", web::get().to(health_check_handler));
}

// router2
// curl -X POST localhost:5000/courses/ -H "Content-Type: application/json" -d '{"teacher_id": 1, "name": "First course"}'
// curl -X GET localhost:5000/courses/1
// curl -X GET localhost:5000/courses/1/1
pub fn course_routes(cfg: &mut web::ServiceConfig) {
    println!("[router][course_routes]===========>");
    cfg.service(
        web::scope("/courses")
            .route("/", web::post().to(new_course))
            .route("/{user_id}", web::get().to(get_courses_for_teacher))
            .route("/{use_id}/{course_id}", web::get().to(get_course_detail))
    );
}
