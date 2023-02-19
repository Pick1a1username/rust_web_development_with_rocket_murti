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
use rocket::form::{self, DataField, Form, FromForm, FromFormField, ValueField};
use uuid::Uuid;

#[derive(sqlx::Type, Debug, FromFormField)]
#[repr(i32)]
enum UserStatus {
    Inactive = 0,
    Active = 1,
}

#[derive(Debug, FromRow, FromForm)]
struct User {
    uuid: OurUuid,
    username: String,
    email: String,
    password_hash: String,
    description: String,
    status: UserStatus,
    created_at: OurDateTime,
    updated_at: OurDateTime,
}

#[derive(Database)]
#[database("main_connection")]
struct DBConnection(PgPool);

type HtmlResponse = Result<RawHtml<String>, Status>;

#[derive(Debug, FromRow)]
struct OurDateTime(DateTime<Utc>);

#[rocket::async_trait]
impl<'r> FromFormField<'r> for OurDateTime {
    fn from_value(_: ValueField<'r>) -> form::Result<'r, Self> {
        todo!("will implement later")
    }
    async fn from_data(_: DataField<'r, '_>) -> form::Result<'r, Self> {
        todo!("will implement later")
    }
}

// uuid::Uuid cannot be used with rocket 0.5.0-rc.2.
// so, try to do something like OurDateTime.
#[derive(Debug, FromRow)]
struct OurUuid(Uuid);

#[rocket::async_trait]
impl<'r> FromFormField<'r> for OurUuid {
    fn from_value(_: ValueField<'r>) -> form::Result<'r, Self> {
        todo!("will implement later")
    }
    async fn from_data(_: DataField<'r, '_>) -> form::Result<'r, Self> {
        todo!("will implement later")
    }
}

#[derive(FromForm)]
struct Pagination {
    cursor: OurDateTime,
    limit: usize,
}

#[get("/users/<_uuid>", format = "text/html")]
async fn get_user(mut _db: Connection<DBConnection>, _uuid: &str) -> HtmlResponse {
    todo!("will implement later")
}

#[get("/users?<_pagination>", format = "text/html")]
async fn get_users(mut _db: Connection<DBConnection>, _pagination: Option<Pagination>) -> HtmlResponse {
    todo!("will implement later")
}

#[get("/users/new", format = "text/html")]
async fn new_user(mut _db: Connection<DBConnection>) -> HtmlResponse {
    todo!("will implement later")
}

#[post("/users", format = "text/html", data = "<_user>")]
async fn create_user(mut _db: Connection<DBConnection>, _user: Form<User>) -> HtmlResponse {
    todo!("will implement later")
}

#[get("/users/edit/<_uuid>", format = "text/html")]
async fn edit_user(mut _db: Connection<DBConnection>, _uuid: &str) -> HtmlResponse {
    todo!("will implement later")
}

#[put("/users/<_uuid>", format = "text/html", data = "<_user>")]
async fn put_user(mut _db: Connection<DBConnection>, _uuid: &str, _user: Form<User>) -> HtmlResponse {
    todo!("will implement later")
}

#[patch("/users/<_uuid>", format = "text/html", data = "<_user>")]
async fn patch_user(mut _db: Connection<DBConnection>, _uuid: &str, _user: Form<User>) -> HtmlResponse {
    todo!("will implement later")
}

#[delete("/users/<_uuid>", format = "text/html")]
async fn delete_user(mut _db: Connection<DBConnection>, _uuid: &str) -> HtmlResponse {
    todo!("will implement later")
}

#[launch]
async fn rocket() -> Rocket<Build> {
    rocket::build().attach(DBConnection::init())
}