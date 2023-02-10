// Todo: Chapter 4: Implement the routine to read the configuration and map it into the rocket() function:

#[macro_use]
extern crate rocket;

use std::io::Cursor;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};

use rocket::{Build, Rocket, State};
use rocket::fs::{NamedFile, relative};
use rocket::response::{self, Responder, Response};
use rocket::request::{FromParam, Request};
use rocket::http::{ContentType, Status};
use serde::Deserialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromForm)]
struct Filters {
    age: u8,
    active: bool,
}

fn default_response<'r>() -> response::Response<'r> {
    Response::build()
        .header(ContentType::Plain)
        .raw_header("X-CUSTOM-ID", "CUSTOM")
        .finalize()
}

#[derive(Debug, FromRow)]
#[sqlx(rename_all = "camelCase")]
struct User {
    uuid: String,
    name: String,
    age: i16,
    grade: i16,
    #[sqlx(rename = "active")]
    present: bool,
    #[sqlx(default)]
    not_in_database: String,
}

impl<'r> Responder<'r, 'r> for &'r User {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'r> {
        let base_response = default_response();
        let user = format!("Found user: {:?}", self);
        Response::build()
            .sized_body(user.len(), Cursor::new(user))
            .raw_header("X-USER-ID", self.uuid.to_string())
            .merge(base_response)
            .ok()
    }
}

struct NameGrade<'r> {
    name: &'r str,
    grade: u8,
}

impl<'r> FromParam<'r> for NameGrade<'r> {
    type Error = &'static str;
    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        const ERROR_MESSAGE: Result<NameGrade, &'static str> = Err("Error parsing user parameter");
        let name_grade_vec: Vec<&'r str> = param.split('_').collect();
        match name_grade_vec.len() {
            2 => match name_grade_vec[1].parse::<u8> (){
                Ok(n) => Ok(Self {
                    name: name_grade_vec[0],
                    grade: n,
                }),
                Err(_) => ERROR_MESSAGE,
            },
            _ => ERROR_MESSAGE,
        }
    }
}

struct NewUser<'a>(Vec<&'a User>);

impl<'r> Responder<'r, 'r> for NewUser<'r> {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'r> {
        let base_response = default_response();
        let user =self
            .0
            .iter()
            .map(|u| format!("{:?}", u))
            .collect::<Vec<String>>()
            .join(",");
        Response::build()
            .sized_body(user.len(), Cursor::new(user))
            .join(base_response)
            .ok()
    }
}

struct VisitorCounter {
    visitor: AtomicU64,
}

impl VisitorCounter {
    fn increment_counter(&self) {
        self.visitor.fetch_add(1, Ordering::Relaxed);
        println!(
            "The number of visitor is: {}", self.visitor.load(Ordering::Relaxed)
        );
    }
}

#[derive(Deserialize)]
struct Config {
    database_url: String,
}

#[route(GET, uri = "/favicon.png")]
async fn favicon() -> NamedFile {
    NamedFile::open(Path::new(relative!("static")).join("favicon.png")).await.unwrap()
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("We cannot find this page {}.", req.uri())
}

#[catch(403)]
fn forbidden(req: &Request) -> String {
    format!("Access forbidden {}.", req.uri())
}

#[route(GET, uri = "/user/<uuid>", rank = 1, format = "text/plain")]
fn user<'a>(counter: &State<VisitorCounter>, uuid: &'a str) -> Option<&'a User> { 
    counter.increment_counter();
    USERS.get(uuid)
}

#[route(GET, uri = "/users/<name_grade>?<filters..>")]
fn users<'a>(counter: &State<VisitorCounter>, name_grade: NameGrade, filters: Option<Filters>) -> Result<NewUser<'a>, Status> {
    counter.increment_counter();
    let users: Vec<&User> = USERS
        .values()
        .filter(|user| user.name.contains(&name_grade.name) && user.grade == name_grade.grade)
        .filter(|user| {
            if let Some(fts) = &filters {
                user.age == fts.age && user.active == fts.active
            } else {
                true
            }
        })
        .collect();
    if users.is_empty() {
        Err(Status::Forbidden)
    } else {
        Ok(NewUser(users))
    }
}

#[launch]
fn rocket() -> Rocket<Build> {
    let visitor_counter = VisitorCounter {
        visitor: AtomicU64::new(0),
    };
    rocket::build()
        .manage(visitor_counter)
        .mount("/", routes![user, users, favicon])
        .register("/", catchers![not_found, forbidden])
}
