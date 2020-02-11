extern crate actix_web;

use actix_web::{get, post, put, web, HttpResponse};
use uuid::Uuid;
use crate::models::user::{User, UserResult, NewUser, UserLogin};
use crate::errors::api_error::ApiError;

#[post("/users")]
async fn create_user(new_user: web::Json<NewUser>) -> Result<HttpResponse, ApiError> {
    let created = User::create(new_user.into_inner())?;
    Ok(HttpResponse::Created().json(UserResult::from(created)))
}

#[put("/users/{id}")]
async fn update_login(id: web::Path<Uuid>, user_login: web::Json<UserLogin>) -> Result<HttpResponse, ApiError> {
    let updated = User::update_login(id.into_inner(), user_login.into_inner())?;
    Ok(HttpResponse::Ok().json(UserResult::from(updated)))
}

#[get("/users")]
async fn get_users() -> Result<HttpResponse, ApiError> {
    let users = User::find_all()?;
    let mapped = users.iter()
        .map(|user: &User| UserResult::from(user))
        .collect::<Vec<_>>();

    Ok(HttpResponse::Ok().json(mapped))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_user);
    cfg.service(get_users);
    cfg.service(update_login);
}