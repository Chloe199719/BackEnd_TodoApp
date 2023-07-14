use actix_web::{ get, HttpRequest, HttpResponse, web};
use serde::{Deserialize, Serialize};

use disel_todo::ops;

use crate::AppState;


#[derive(Deserialize, Serialize, Debug)]
pub struct SessionCheckStruct {
    id: uuid::Uuid,
    username: String,
    email: String,
    email_verified: bool,
}

#[get("/session_check")]


pub async fn session_check(req: HttpRequest , data:web::Data<AppState>)-> HttpResponse {
   let cookie_value = match req.cookie( "session") {
        Some(cookie) => cookie.value().to_string(),
        None => return HttpResponse::Ok().json("No Session".to_string()),
   };
   let x = &data.connection;
  
    let session = disel_todo::ops::get_user_by_session2(cookie_value.clone(), x ).unwrap();
    if session.0.expires_at < chrono::Utc::now().naive_utc() {
        ops::session_delete(cookie_value).unwrap();
        return HttpResponse::Accepted().json("Session Expired".to_string());
    }

    
    HttpResponse::Ok().json(SessionCheckStruct {
        id: session.1.id,
        username: session.1.username,
        email: session.1.email,
        email_verified: session.1.email_verified,
    })
}