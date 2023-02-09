// Todo: Chapter 3 Built-in implementationsssh-

#[macro_use]
extern crate rocket;

use std::collections::HashMap;
use std::io::Cursor;
use std::path::Path;

use rocket::{Build, Rocket};
use rocket::fs::{NamedFile, relative};
use rocket::response::{self, Responder, Response};
use rocket::request::{FromParam, Request};
use rocket::http::{ContentType, Status};
use lazy_static::lazy_static;

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

#[derive(Debug)]
struct User {
    uuid: String,
    name: String,
    age: u8,
    grade: u8,
    active: bool,
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

lazy_static! {
    static ref USERS: HashMap<&'static str, User> = {
        let mut map = HashMap::new();
        map.insert(
            "3e3dd4ae-3c37-40c6-aa64-7061f284ce28",
            User {
                uuid: "3e3dd4ae-3c37-40c6-aa64-7061f284ce28".to_string(),
                name: "John".to_string(),
                age: 8,
                grade: 99,
                active: true,
            },      
        );
        map
    };
}

#[route(GET, uri = "/user/<uuid>", rank = 1, format = "text/plain")]
fn user(uuid: &str) -> Option<&User> { 
    USERS.get(uuid)
}

#[route(GET, uri = "/users/<name_grade>?<filters..>")]
fn users(name_grade: NameGrade, filters: Option<Filters>) -> Result<NewUser, Status> {
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
    rocket::build().mount("/", routes![user, users, favicon])
        .register("/", catchers![not_found, forbidden])
}
