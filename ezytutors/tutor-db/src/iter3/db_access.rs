use super::models::Course;
use sqlx::postgres::PgPool;

pub async fn get_course_for_tutor_db(pool: &PgPool, tutor_id: i32) -> Vec<Course> {
    let course_rows = sqlx::query!(
        "SELECT tutor_id, course_id, course_name, posted_time FROM course WHERE tutor_id = $1",
        tutor_id
    )
    .fetch_all(pool)
    .await
    .unwrap();

    course_rows
        .iter()
        .map(|row| Course {
            course_id: row.course_id,
            course_name: row.course_name.clone(),
            posted_time: Some(chrono::NaiveDateTime::from(row.posted_time.unwrap())),
            tutor_id: row.tutor_id,
        })
        .collect()
}

pub async fn get_course_detail_db(pool: &PgPool, tutor_id: i32, course_id: i32) -> Course {
    let row = sqlx::query!("SELECT tutor_id, course_id, course_name, posted_time FROM course WHERE tutor_id = $1 and course_id = $2", tutor_id, course_id).fetch_one(pool).await.unwrap();

    Course {
        tutor_id: row.tutor_id,
        course_id: row.course_id,
        course_name: row.course_name,
        posted_time: row.posted_time,
    }
}

pub async fn post_new_course_db(pool: &PgPool, course: Course) -> Course {
    let row = sqlx::query!(
        "insert into course (
   course_id,tutor_id, course_name) values ($1,$2,$3) returning
   tutor_id, course_id,course_name, posted_time",
        course.course_id,
        course.tutor_id,
        course.course_name
    )
    .fetch_one(pool)
    .await
    .unwrap();

    Course {
        course_id: row.course_id,
        tutor_id: row.tutor_id,
        course_name: row.course_name,
        posted_time: Some(chrono::NaiveDateTime::from(row.posted_time.unwrap())),
    }
}
