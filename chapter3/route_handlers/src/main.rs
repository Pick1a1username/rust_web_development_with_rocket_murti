#[macro_use]
extern crate rocket;

use std::collections::HashMap;
use rocket::{FromParam, Build, Rocket};
use lazy_static::lazy_static;

#[derive(FromForm)]
struct Filters {
    age: u8,
    active: bool,
}

#[derive(Debug)]
struct User {
    uuid: String,
    name: String,
    age: u8,
    grade: u8,
    active: bool,
}

struct NameGrade<'r> {
    name: &'r str,
    grades: u8,
}

impl<'r> FromParam<'r> for NameGrade<'r> {
    type Error = &'static str;
    fn from_param(param: &'r str) -> Result<Self, Self::Error> {} // Todo
}

lazy_static! {
    static ref USERS: HashMap<&'static str, User> = {
        let mut map = HashMap::new();
        map.insert(
            "3e3dd4ae-3c37-40c6-aa64-7061f284ce28",
            User {
                uuid: "3e3dd4ae-3c37-40c6-aa64-7061f284ce28".to_string(),
                name: "John Doe".to_string(),
                age: 8,
                grade: 99,
                active: true,
            },      
        );
        map
    };
}

#[route(GET, uri = "/user/<uuid>", rank = 1, format = "text/plain")]
fn user(uuid: &str) -> String { 
    let user = USERS.get(uuid);
    match user {
        Some(u) => format!("Found user: {:?}", u),
        None => String::from("User not found"),
    }
}

#[route(GET, uri = "/user/<name_grade>?<filters..>")]
fn users(name_grade: u8, filters: Filters) { /* ... */}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![user, users])
}
