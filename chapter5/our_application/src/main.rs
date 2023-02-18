#[macro_use]
extern crate rocket;

use chrono::{offset::Utc, DateTime};
use rocket::{Build, Rocket};
use rocket::http::Status;
use rocket::response::content::RawHtml;
use rocket_db_pools::{
    sqlx::{FromRow, PgPool},
    Connection,
    Database
};
use uuid::Uuid;

#[derive(sqlx::Type, Debug)]
#[repr(i32)]
enum UserStatus {
    Inactive = 0,
    Active = 1,
}

#[derive(Debug, FromRow)]
struct User {
    uuid: Uuid,
    username: String,
    email: String,
    password_hash: String,
    description: String,
    status: UserStatus,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Database)]
#[database("main_connection")]
struct DBConnection(PgPool);

type HtmlResponse = Result<RawHtml<String>, Status>;

#[get("/users/<_uuid>", format = "text/html")]
async fn get_user(mut _db: Connection<DBConnection>, _uuid: &str) -> HtmlResponse {
    todo!("will implement later")
}

#[launch]
async fn rocket() -> Rocket<Build> {
    rocket::build().attach(DBConnection::init())
}