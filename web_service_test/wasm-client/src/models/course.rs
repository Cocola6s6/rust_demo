use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Course {
    pub id: i32,    // TODO 什么时候结构体字段需要Option类型？
    pub course_name: String,
    pub teacher_id: i32,
    pub create_time: NaiveDate,
}
