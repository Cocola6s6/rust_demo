use crate::dbaccess::course::*;
use crate::errors::MyError;
use crate::models::course::Course;
use crate::state::AppState;
use actix_web::{web, HttpResponse};

// handler2
pub async fn new_course(
    new_course: web::Json<Course>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    println!("Received new course");
    let course = post_new_course_db(&app_state.db, new_course.into()).await;
    println!("[handler][new_course]==============>");
    Ok(HttpResponse::Ok().json("Success"))
}

// handler3
pub async fn get_courses_for_teacher(
    app_state: web::Data<AppState>,
    params: web::Path<(usize,)>,
) -> Result<HttpResponse, MyError> {
    // TODO 为什么可以使用Result<HttpResponse, MyError>作为返回，不是需要返回Responder类型吗？
    // 因为Result<T, E>实现了Responder trait，T的特征约束是Response, E的特征约束是Error，所以HttpResponse需要实现Response，MyError需要实现Error
    let teacher_id = i32::try_from(params.0).unwrap();
    let courses = get_courses_for_teacher_db(&app_state.db, teacher_id).await;
    match courses.len() {
        0 => Err(MyError::NotFound("No courses found for teacher".to_string()).into()),
        _ => Ok(HttpResponse::Ok().json(courses)),
    }
}

// handler4
pub async fn get_course_detail(
    app_state: web::Data<AppState>,
    params: web::Path<(usize, usize)>,
) -> Result<HttpResponse, MyError> {
    let teacher_id = i32::try_from(params.0).unwrap();
    let course_id = i32::try_from(params.1).unwrap();
    let course = get_course_detail_db(&app_state.db, teacher_id, course_id).await;


    if let Ok(course) = course {
        Ok(HttpResponse::Ok().json(course))
    } else {
        Err(MyError::NotFound("Course not found".to_string()).into())
    }
}
