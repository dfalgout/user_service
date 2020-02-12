extern crate actix_web;

use actix_web::{get, post, put, web, HttpResponse};
use uuid::Uuid;
use crate::models::user::{User, UserLogin, RegisterUser};
use crate::errors::api_error::ApiError;

#[post("/users")]
async fn create_user(register_user: web::Json<RegisterUser>) -> Result<HttpResponse, ApiError> {
    let validated = register_user
        .into_inner()
        .validates()?;
    let created = User::create(validated)?;

    Ok(HttpResponse::Created().json(created))
}

#[put("/users/{id}")]
async fn update_login(id: web::Path<Uuid>, user_login: web::Json<UserLogin>) -> Result<HttpResponse, ApiError> {
    let updated = User::update_login(id.into_inner(), user_login.into_inner())?;
    Ok(HttpResponse::Ok().json(updated))
}

#[get("/users")]
async fn get_users() -> Result<HttpResponse, ApiError> {
    let users = User::find_all()?;
    let mapped = users.iter()
        .map(|user: &User| user)
        .collect::<Vec<_>>();

    Ok(HttpResponse::Ok().json(mapped))
}

#[get("/users/{id}")]
async fn get_user(id: web::Path<Uuid>) -> Result<HttpResponse, ApiError> {
    let user = User::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}

// #[post("/login")]
// async fn login(req: web::HttpRequest) -> Result<HttpResponse, ApiError> {
//     match req.headers().get("Authorization") {
//         Some(header) => Ok(HttpResponse::Ok().body(header::from)),
//         None => Ok(HttpResponse::InternalServerError().body("failed")),
//     }
// }

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_user);
    cfg.service(get_users);
    cfg.service(update_login);
    cfg.service(get_user);
}