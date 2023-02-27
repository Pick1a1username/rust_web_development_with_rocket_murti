use std::error::Error;
use uuid::Uuid;

use super::our_date_time::OurDateTime;
use super::our_uuid::OurUuid;
use super::user_status::UserStatus;
use rocket::form::FromForm;
use rocket_db_pools::sqlx::{FromRow, PgConnection};

#[derive(Debug, FromRow, FromForm)]
pub struct User {
    pub uuid: OurUuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub description: String,
    pub status: UserStatus,
    pub created_at: OurDateTime,
    pub updated_at: OurDateTime,
}

impl User{
    pub async fn find(connection: &mut PgConnection, uuid: &str) -> Result<Self, Box<dyn Error>> {
        let parsed_uuid = OurUuid(Uuid::parse_str(uuid)?);
        let query_str = "SELECT * FROM users WHERE uuid = $1";
        Ok(sqlx::query_as::<_, Self>(query_str)
            .bind(parsed_uuid)
            .fetch_one(connection)
            .await?)
    }
}