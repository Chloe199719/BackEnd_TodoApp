use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use backend_todo::ops::{get_user, create_user,login};


use std::sync::Mutex;


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



struct AppState{
    called: Mutex<u32>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppState{
        called: Mutex::new(0),
    });
    HttpServer::new(move || App::new().app_data(counter.clone())
    .service(index)
    .service(count)
    .service(delete_count)
    .service(get_user::user_by_id)
    .service(create_user::create_user)
    .service(login::login))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
