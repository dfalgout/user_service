use diesel::{Queryable, Insertable};
use uuid::Uuid;
use serde::{Deserialize, Serialize};

use crate::schema::users;
use crate::config::database::connection;
use crate::errors::api_error::ApiError;

use diesel::prelude::*;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub user_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Insertable, Queryable, Deserialize)]
#[table_name="users"]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, AsChangeset)]
#[table_name="users"]
pub struct UserLogin {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct UserResult {
    pub user_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

impl User {
    pub fn create(new_user: NewUser) -> Result<Self, ApiError> {
        let conn = connection()?;

        match Self::exists(new_user.email.clone())? {
            Some(_) => Err(ApiError::new(400, "A user with that email already exists".to_string())),
            None => {
                let user = diesel::insert_into(users::table)
                    .values(new_user)
                    .get_result(&conn)?;
    
                Ok(user)
            }
        }
    }

    pub fn update_login(id: Uuid, user_login: UserLogin) -> Result<Self, ApiError> {
        let conn = connection()?;

        match Self::exists(user_login.email.clone())? {
            Some(_) => Err(ApiError::new(400, "A user with that email already exists".to_string())),
            None => {
                let user = diesel::update(users::table)
                    .filter(users::user_id.eq(id))
                    .set(user_login)
                    .get_result(&conn)?;
            
                Ok(user)
            }
        }
    }

    pub fn find_all() -> Result<Vec<Self>, ApiError> {
        let conn = connection()?;

        let users = users::table
            .load::<User>(&conn)?;
        
        Ok(users)
    }

    fn exists(email: String) -> Result<Option<Self>, ApiError> {
        let conn = connection()?;

        let found = users::table
            .filter(users::email.eq(email))
            .first::<User>(&conn)
            .optional()?;
        
        Ok(found)
    }
}

impl From<User> for UserResult {
    fn from(user: User) -> Self {
        UserResult {
            user_id: user.user_id,
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
        }
    }
}

impl From<&User> for UserResult {
    fn from(user: &User) -> Self {
        UserResult {
            user_id: user.user_id,
            first_name: String::from(&user.first_name),
            last_name: String::from(&user.last_name),
            email: String::from(&user.email),
        }
    }
}