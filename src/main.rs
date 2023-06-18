use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use model::student::Student;
use rusqlite::Connection;

mod handeler;
mod model;

pub struct AppState {
    db: Connection,
}

#[actix_web::get("/")]
async fn echo () -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() {
    HttpServer::new(|| {
        let db = Connection::open("db.sqlite").unwrap();
        Student::init(&db).unwrap();
        App::new()
            .app_data(web::Data::new(AppState {
                db
            }))
            .service(handeler::student_handeler::create_student)
            .service(handeler::student_handeler::get_student)
            .service(handeler::student_handeler::delete_student)
            .service(handeler::student_handeler::update_student)
            .service(echo)
    })
    .bind(("127.0.0.1", 8081))
    .unwrap()
    .run()
    .await
    .unwrap();
}
