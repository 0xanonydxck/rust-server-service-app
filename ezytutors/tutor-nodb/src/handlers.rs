use crate::models::Course;

use super::state::AppState;
use actix_web::{web, HttpResponse};
use chrono::Utc;

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}

pub async fn new_course(course: web::Json<Course>, app_state: web::Data<AppState>) -> HttpResponse {
    let courses_count_for_user = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|c| c.tutor_id == course.tutor_id)
        .count();

    let new_course = Course {
        tutor_id: course.tutor_id,
        course_id: Some(courses_count_for_user as i32 + 1),
        course_name: course.course_name.clone(),
        posted_time: Some(Utc::now().naive_utc()),
    };
    app_state.courses.lock().unwrap().push(new_course.clone());
    HttpResponse::Created().json(&new_course)
}

pub async fn get_courses_for_tutor(
    params: web::Path<(i32,)>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    let tutor_id: i32 = params.0;
    let courses = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|c| c.tutor_id == tutor_id)
        .collect::<Vec<Course>>();

    if courses.len() > 0 {
        HttpResponse::Ok().json(&courses)
    } else {
        HttpResponse::NotFound().json("no courses found for tutor".to_string())
    }
}

pub async fn get_course_detail(
    params: web::Path<(i32, i32)>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    let tutor_id: i32 = params.0;
    let course_id: i32 = params.1;

    let course = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .find(|c| c.tutor_id == tutor_id && c.course_id == Some(course_id))
        .ok_or("course not found");

    if let Ok(c) = course {
        HttpResponse::Ok().json(&c)
    } else {
        HttpResponse::NotFound().json("course not found".to_string())
    }
}
