use actix_web::web;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Course {
    pub id: Option<i32>,
    pub course_name: Option<String>,
    pub teacher_id: Option<i32>,
    pub create_time: Option<NaiveDate>,
}

impl From<web::Json<Course>> for Course {
    fn from(course: web::Json<Course>) -> Self {
        Course {
            id: course.id,
            course_name: course.course_name.clone(),
            teacher_id: course.teacher_id,
            create_time: course.create_time,
        }
    }
}