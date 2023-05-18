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
    let course = post_new_course_db(&app_state.db, new_course.into()).await?;   // 同样的，继续使用传播错误运算符将错误自动转换为自定义，并且传给上层
    Ok(HttpResponse::Ok().json(course))  // 同样的，传播错误运算符只处理了panic返回，Result的正确返回需要Ok()处理
}

// handler3
pub async fn get_courses_for_teacher(
    app_state: web::Data<AppState>,
    params: web::Path<(usize,)>,
) -> Result<HttpResponse, MyError> {
    // TODO 为什么可以使用Result<HttpResponse, MyError>作为返回，不是需要返回Responder类型吗？
    // 因为Result<T, E>实现了Responder trait，T的特征约束是Response, E的特征约束是Error，所以HttpResponse需要实现Response，MyError需要实现Error
    let teacher_id = i32::try_from(params.0).unwrap(); // TOOD 为什么这样获取路径值？之前是：let teacher_id: usize = params.0;
    let courses = get_courses_for_teacher_db(&app_state.db, teacher_id).await?;
    Ok(HttpResponse::Ok().json(courses))
}

// handler4
pub async fn get_course_detail(
    app_state: web::Data<AppState>,
    params: web::Path<(usize, usize)>,
) -> Result<HttpResponse, MyError> {
    let teacher_id = i32::try_from(params.0).unwrap();  // TOOD 为什么这样获取路径值？之前是：let (teacher_id, course_id) = params.0;
    let id = i32::try_from(params.1).unwrap();  // TOOD 为什么这样获取路径值？

    let course = get_course_detail_db(&app_state.db, teacher_id, id).await?;
    Ok(HttpResponse::Ok().json(course))
}
