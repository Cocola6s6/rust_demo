use super::errors::MyError;
use super::models::Course;
use super::state::AppState;
use actix_web::{web, HttpResponse};
use chrono::Utc;

// handler1
pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();

    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}

// handler2
pub async fn new_course(
    new_course: web::Json<Course>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    println!("Received new course");
    let course_count = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|course| course.teacher_id == new_course.teacher_id)
        .collect::<Vec<Course>>()
        .len();
    let new_course = Course {
        teacher_id: new_course.teacher_id,
        id: Some(course_count + 1),
        name: new_course.name.clone(),
        time: Some(Utc::now().naive_utc()),
    };
    app_state.courses.lock().unwrap().push(new_course);
    println!("[handler][new_course]==============>");
    Ok(HttpResponse::Ok().json("Course added"))
}

// handler3
pub async fn get_courses_for_teacher(
    app_state: web::Data<AppState>,
    params: web::Path<(usize)>,
) -> Result<HttpResponse, MyError> {    // TODO 为什么可以使用Result<HttpResponse, MyError>作为返回，不是需要返回Responder类型吗？
                                        // 因为Result<T, E>实现了Responder trait，T的特征约束是Response, E的特征约束是Error，所以HttpResponse需要实现Response，MyError需要实现Error
    let teacher_id: usize = params.0;
    let filtered_courses = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|course| course.teacher_id == teacher_id)
        .collect::<Vec<Course>>();

    match filtered_courses.len() {
        0 => Err(MyError::NotFound("No courses found for teacher".to_string()).into()),
        _ => Ok(HttpResponse::Ok().json(filtered_courses)),
    }
}

// handler4
pub async fn get_course_detail(
    app_state: web::Data<AppState>,
    params: web::Path<(usize, usize)>,
) -> Result<HttpResponse, MyError> {
    let (teacher_id, course_id) = params.0;
    let selected_course = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .find(|x| x.teacher_id == teacher_id && x.id == Some(course_id))
        .ok_or("Course not found");


    if let Ok(course) = selected_course {
        Ok(HttpResponse::Ok().json(course))
    } else {
        Err(MyError::NotFound("Course not found".to_string()).into())
    }
}
