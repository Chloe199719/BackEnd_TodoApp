use actix_web::{web::Json, post};

use disel_todo::ops;
use disel_todo::models;
use serde::{Deserialize, Serialize};
use crypto_hash::{Algorithm, hex_digest};
#[derive(Deserialize, Serialize, Debug)]
pub struct CreateUserStruct {
    username: String,
    email: String,
    password: String,
}

#[post("/create_user")]

pub async fn create_user(body:Json<CreateUserStruct>)-> Json<String> {
    let hashed_password = hex_digest(Algorithm::SHA256, body.password.as_bytes());
    ops::create_user(models::NewUsers {
        username: body.username.clone(),
        email: body.email.clone(),
        password: hashed_password,
    }).unwrap();
    Json("User Created".to_string())
}

