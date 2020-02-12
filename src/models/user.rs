use diesel::{Queryable, Insertable};
use uuid::Uuid;
use chrono::{NaiveDateTime, Local};
use serde::{Deserialize, Serialize};

use crate::schema::users;
use crate::config::database::connection;
use crate::errors::api_error::ApiError;

use diesel::prelude::*;
use bcrypt::{DEFAULT_COST, verify, hash};

#[derive(Queryable, Serialize)]
pub struct User {
    #[serde(skip)]
    pub id: i32,
    pub user_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    #[serde(skip)]
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Queryable)]
#[table_name="users"]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct RegisterUser {
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

impl RegisterUser {
    pub fn validates(self) -> Result<RegisterUser, ApiError> {
        let conn = connection()?;

        match users::table
            .filter(users::email.eq(self.email.clone()))
            .first::<User>(&conn)
            .optional()? {
                Some(_) => Err(ApiError::new(400, "A user with that email already exists".to_string())),
                None => Ok(self),
            }
    }
}

impl User {
    pub fn create(register_user: RegisterUser) -> Result<Self, ApiError> {
        let conn = connection()?;

        let user = diesel::insert_into(users::table)
            .values(NewUser {
                first_name: register_user.first_name,
                last_name: register_user.last_name,
                email: register_user.email,
                password: Self::hash_password(register_user.password)?,
                created_at: Local::now().naive_local(),
                updated_at: Local::now().naive_local(),
            })
            .get_result(&conn)?;

        Ok(user)
    }

    pub fn update_login(id: Uuid, user_login: UserLogin) -> Result<Self, ApiError> {
        let conn = connection()?;

        match Self::exists(user_login.email.clone())? {
            Some(_) => Err(ApiError::new(400, "A user with that email already exists".to_string())),
            None => {
                let user = diesel::update(users::table)
                    .filter(users::user_id.eq(id))
                    .set(UserLogin {
                        email: user_login.email,
                        password: Self::hash_password(user_login.password)?,
                    })
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

    pub fn find(id: Uuid) -> Result<Self, ApiError> {
        let conn = connection()?;

        let user = users::table
            .filter(users::user_id.eq(id))
            .first::<User>(&conn)?;

        Ok(user)
    }

    fn exists(email: String) -> Result<Option<Self>, ApiError> {
        let conn = connection()?;

        let found = users::table
            .filter(users::email.eq(email))
            .first::<User>(&conn)
            .optional()?;
        
        Ok(found)
    }

    fn hash_password(password: String) -> Result<String, ApiError> {
        Ok(hash(password, DEFAULT_COST)?)
    }

    pub fn login(email: String, password: String) -> Result<bool, ApiError> {
        match Self::exists(email)? {
            Some(user) => {
                Ok(verify(password, &user.password)?)
            },
            None => Ok(false),
        }
    }
}