use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
// use crate::models::course::Course;
use crate::errors::MyError;
use std::convert::TryFrom;

#[derive(Serialize, Debug, Clone, sqlx::FromRow)]
pub struct Course {
    pub id: i32,
    pub teacher_id: i32,
    pub name: String,
    pub time: Option<NaiveDateTime>,
}

#[derive(Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct CreateCourse {
    pub teacher_id: i32,
    pub name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UpdateCourse {
    pub name: Option<String>,
}

// impl From<web::Json<Course>> for CreateCourse {
//     fn from(course: web::Json<Course>) -> Self {
//         CreateCourse {
//             teacher_id: course.teacher_id,
//             name: course.name.clone(),
//             description: course.description.clone(),
//             format: course.format.clone(),
//             structure: course.structure.clone(),
//             duration: course.duration.clone(),
//             price: course.price.clone(),
//             language: course.language.clone(),
//             level: course.level.clone(),
//         }
//     }
// }

impl TryFrom<web::Json<CreateCourse>> for CreateCourse {
    type Error = MyError;

    fn try_from(course: web::Json<CreateCourse>) -> Result<Self, Self::Error> {
        Ok(CreateCourse {
            teacher_id: course.teacher_id,
            name: course.name.clone(),
        })
    }
}

impl From<web::Json<UpdateCourse>> for UpdateCourse {
    fn from(course: web::Json<UpdateCourse>) -> Self {
        UpdateCourse {
            name: course.name.clone(),
        }
    }
}
