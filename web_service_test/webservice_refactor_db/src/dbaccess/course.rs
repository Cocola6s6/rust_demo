use crate::errors::MyError;
use crate::models::course::Course;
use chrono::NaiveDate;
use sqlx::postgres::PgPool;

pub async fn post_new_course_db(pool: &PgPool, new_course: Course) -> Result<Course, MyError> {
    // 引入DB后，会引发SQLError导致panic，所以将返回值改为定义错误返回
    println!("[post_new_course_db]");
    let row = sqlx::query!(
        r#"INSERT INTO course(teacher_id, course_name)
        VALUES ($1, $2)
        RETURNING id, teacher_id, course_name, create_time"#,
        new_course.teacher_id,
        new_course.course_name,
    )
    .fetch_one(pool)
    .await?; // 不再使用unwrap直接panic，使用传播错误运算符将错误自动转换为自定义，并且传给上层

    Ok(Course {
        // 传播错误运算符只处理了panic返回，Result的正确返回需要Ok()处理
        id: Some(row.id),
        teacher_id: row.teacher_id,
        course_name: row.course_name.clone(),
        create_time: Some(NaiveDate::from(row.create_time.unwrap())),
    })
}

pub async fn get_courses_for_teacher_db(
    pool: &PgPool,
    teacher_id: i32,
) -> Result<Vec<Course>, MyError> {
    println!("[get_courses_for_teacher_db]");
    let rows = sqlx::query!(
        r#"SELECT id, teacher_id, course_name, create_time
        FROM course
        WHERE teacher_id = $1"#,
        teacher_id
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .iter()
        .map(|r| Course {
            id: Some(r.id),
            teacher_id: r.teacher_id,
            course_name: r.course_name.clone(),
            create_time: Some(NaiveDate::from(r.create_time.unwrap())),
        })
        .collect())
}

pub async fn get_course_detail_db(
    pool: &PgPool,
    teacher_id: i32,
    id: i32,
) -> Result<Course, MyError> {
    println!("[get_course_detail_db]");
    let row = sqlx::query!(
        r#"SELECT id, teacher_id, course_name, create_time
        FROM course
        WHERE teacher_id = $1 AND id = $2"#,
        teacher_id,
        id
    )
    .fetch_one(pool) // TODO fetch_one和fetch_alls的区别是什么
    .await?;

    Ok(Course {
        id: Some(row.id),
        teacher_id: row.teacher_id,
        course_name: row.course_name.clone(),
        create_time: Some(NaiveDate::from(row.create_time.unwrap())),
    })
}

pub async fn get_courses_all_db(pool: &PgPool) -> Result<Vec<Course>, MyError> {
    println!("[get_courses_all_db]");
    let rows = sqlx::query!(
        r#"SELECT id, teacher_id, course_name, create_time
        FROM course"#
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .iter()
        .map(|r| Course {
            id: Some(r.id),
            teacher_id: r.teacher_id,
            course_name: r.course_name.clone(),
            create_time: Some(NaiveDate::from(r.create_time.unwrap())),
        })
        .collect())
}

pub async fn delete_course_db(pool: &PgPool, id: i32) -> Result<i32, MyError> {
    println!("[delete_course]");
    let row = sqlx::query!(
        r#"DELETE
        FROM course
        WHERE id = $1
        RETURNING id"#,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(row.id)
}
