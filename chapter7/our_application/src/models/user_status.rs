use std::fmt;

use rocket::form::FromFormField;
use rocket_db_pools::sqlx;

#[derive(sqlx::Type, Debug, FromFormField)]
#[repr(i32)]
pub enum UserStatus {
    Inactive = 0,
    Active = 1,
}

impl fmt::Display for UserStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            UserStatus::Inactive => write!(f, "Inactive"),
            UserStatus::Active => write!(f, "Active"),
        }
    }
}