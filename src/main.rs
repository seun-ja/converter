#![feature(proc_macro_hygiene, decl_macro, plugin)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

use rocket::request::Request;
use std::collections::HashMap;
use rocket::http::RawStr;
use rocket::response::status;
use rocket::response::{Responder, Response};
use rocket::http::{ContentType, Status};
use rocket_contrib::json::{Json, JsonValue};
use std::io;
use rocket_contrib::templates::Template;
use rocket::response;
use rocket::data::FromData;
// use chrono::{DateTime, TimeZone, Utc};


#[derive(Debug)]
struct ApiResponse {
    json: JsonValue,
    status: Status,
}

impl<'r> Responder<'r> for ApiResponse {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Response::build_from(self.json.respond_to(&req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}

#[post("/converted", format = "application/json", data = "<data>")]
fn home(data: Json<Input>) -> ApiResponse {
    let mut errors: Vec<String> = vec![];

    if errors.len() > 0 {
        let error_response_json = json!({ "errors": errors });
        ApiResponse {
            status: Status::BadRequest,
            json: error_response_json,
        }
    } else {
        ApiResponse {
            status: Status::Created,
            json: json!({"status": "ok"}),
        }
    }
}

#[get("/")]
fn index() -> Template {
    let context: HashMap<String, String> = HashMap::new();
    Template::render("index", &context)
}

#[derive(Debug, FromForm)]
struct Input {
    degree: u32
}

impl Input {
    fn degree_converter(&self) -> f32 {
        (self.degree - 32) as f32 * 0.556
    
        // self.new
    }
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index,home])
        .attach(Template::fairing())
        .launch();

    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let input: u32 = input.trim().parse().expect("Please input interger");

    let fahrenheit = Input {degree: input};

    print!("fahrenheit = {}", fahrenheit.degree_converter());
}


