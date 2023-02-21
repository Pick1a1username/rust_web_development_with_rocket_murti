use super::our_date_time::OurDateTime;
use super::our_uuid::OurUuid;
use super::user_status::UserStatus;
use rocket::form::FromForm;
use rocket_db_pools::sqlx::FromRow;

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