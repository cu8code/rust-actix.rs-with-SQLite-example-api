use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::json;

use crate::{model::student::Student, AppState};

#[post("/student")]
pub async fn create_student(req_body: String, state: web::Data<AppState>) -> impl Responder {
    let student: Student = serde_json::from_str(&req_body).unwrap();
    match Student::create(&state.db, student) {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}

#[get("/student/{id}")]
pub async fn get_student(path: web::Path<(u32,)>, state: web::Data<AppState>) -> impl Responder {
    let enrollment_no = path.into_inner().0;

    match Student::read(&state.db, enrollment_no) {
        Ok(s) => HttpResponse::Ok().json(json!(s)),
        Err(_) => HttpResponse::InternalServerError().json(json!("Error")),
    }
}

#[delete("/student/{id}")]
pub async fn delete_student(path: web::Path<(u32,)>, state: web::Data<AppState>) -> impl Responder {
    let enrollment_no = path.into_inner().0;

    match Student::delete(&state.db, enrollment_no) {
        Ok(_) => HttpResponse::Ok().json("deleted"),
        Err(_) => HttpResponse::InternalServerError().json("did you eneter the right id?"),
    }
}

#[derive(Deserialize)]
struct UpdateStudent {
    name: String,
    email: String,
}

#[patch("/student/{id}")]
pub async fn update_student(req_body: String ,path: web::Path<(u32,)>, state: web::Data<AppState>) -> impl Responder {
    let enrollment_no = path.into_inner().0;

    let json : UpdateStudent = serde_json::from_str(&req_body).unwrap();

    match Student::update(&state.db, enrollment_no, json.name, json.email) {
        Ok(_) => HttpResponse::Ok().json("updated"),
        Err(_) => HttpResponse::InternalServerError().json("did you eneter the right id?")
    }
}
