use actix_web::{web::Path , web::Json, get};
use disel_todo:: ops::get_user_by_id;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
 pub struct UserIdentity {
    id: String,
}
#[derive(Deserialize, Serialize)]
pub struct User {
    id: String,
    username: String,
    email: String,
    password: String,
    email_verified: bool,
}


#[get("/user/{id}")]
pub async fn user_by_id(id: Path<UserIdentity>) -> Json<User> {
    let user = get_user_by_id(uuid::Uuid::parse_str(id.into_inner().id.as_str()).unwrap()).unwrap();
    let user = User {
        id: user.id.to_string(),
        username: user.username,
        email: user.email,
        password: user.password,
        email_verified: user.email_verified,
    };
    Json(user)
}


