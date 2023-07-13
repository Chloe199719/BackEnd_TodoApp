use actix_web::{web::Json, post, HttpResponse, cookie::{time::{Duration, OffsetDateTime}, SameSite} };
use disel_todo::{ops, models::NewSessions};
use ring::{rand::{self, SecureRandom}, digest::{digest, self}};
use serde::{Deserialize, Serialize};
use crypto_hash::{Algorithm, hex_digest};

#[derive(Deserialize, Serialize, Debug)] 
pub struct LoginEmail {
    pub email: String,
    pub password: String,
}


#[post("/login")]
pub async fn login(body: Json<LoginEmail>) -> HttpResponse {
    let hashed_password = hex_digest(Algorithm::SHA256, body.password.as_bytes());
    let user = disel_todo::ops::get_user_by_email(body.email.clone()).unwrap(); // Error Handling Required

    if user.password != hashed_password {
        return HttpResponse::Unauthorized().body("Invalid Password");
    }
    let rng = rand::SystemRandom::new();
    let mut key = [0u8; 32];
    rng.fill(&mut key).unwrap();
    let hashed_value =digest(&digest::SHA256, &key);
    let hex_string = hashed_value
    .as_ref()
    .iter()
    .map(|byte| format!("{:02x}", byte))
    .collect::<Vec<String>>()
    .join("");
    let expire_time = chrono::Utc::now().naive_utc() + chrono::Duration::days(7);
    let test = OffsetDateTime::from_unix_timestamp(expire_time.timestamp()); // Error Handling Required 
     ops::init_session(NewSessions {
        token: hex_string.clone(),
        expires_at: chrono::Utc::now().naive_utc() + chrono::Duration::days(7),
        user_id: user.id,
    }).unwrap(); // Error Handling Required

    let response =  HttpResponse::Ok().cookie( actix_web::cookie::Cookie::build("session", hex_string).http_only(true).http_only(true).same_site(SameSite::Strict).expires(test.unwrap()).max_age(Duration::days(7)).finish()).body("Logged In");

    return response;
}