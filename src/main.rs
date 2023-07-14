use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use backend_todo::ops::{get_user, create_user,login, session_check::session_check};
use backend_todo;
use disel_todo;
use dotenvy::dotenv;
use std::env;
use std::sync::Mutex;

use backend_todo::AppState;
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/count")]
async fn count(data: web::Data<AppState>) -> impl Responder {
    let mut called = data.called.lock().unwrap();
    *called += 1;


    HttpResponse::Ok().body(format!("called: {}", called))
}
#[get("deleteCount")]
async fn delete_count(data: web::Data<AppState>) -> impl Responder {
    let mut called = data.called.lock().unwrap();
    *called = 0;
    HttpResponse::Ok().body(format!("called: {}", called))
}





#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let init = create_con();
    let conn = Mutex::new(init);
    let counter = web::Data::new(AppState{
        connection: conn,
        called: Mutex::new(0),
    });
    HttpServer::new(move || App::new().app_data(counter.clone())
    .service(index)
    .service(count)
    .service(delete_count)
    .service(get_user::user_by_id)
    .service(create_user::create_user)
    .service(login::login)
    .service(session_check))
    
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

fn create_con () -> disel_todo::PgConnection {
    disel_todo::establish_connection().unwrap_or_else(|_| panic!("Error connecting to {}", "postgres://postgres:password@localhost:5432/postgres"))
}