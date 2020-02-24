#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use rocket::response::content::Json;
use std::io;
use rocket::request::Form;
use rocket::http::RawStr;

// use std::ops::Mul;

#[get("/", format = "json")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello?wave&<name>")]
fn hello(name: String) -> String {
    format!("Hello, {}!", name.as_str())
}

#[derive(Debug, PartialEq, FromForm)]
struct Input {
    value: u32
}

// #[post("/submit", data = "<user_input>")]
// fn submit_task(user_input: Form<UserInput>) -> String {
//     format!("Your value: {}", user_input.value)
// }

impl Input {
    fn degree_converter(&self) -> f32 {
        (self.value - 32) as f32 * 0.556
    
        // self.new
    }
}

// #[post("/submit", data = "<user_input>")]
// fn degree_converter(user_input: Form<UserInput>) -> f32 {
//     (user_input.value - 32) as f32 * 0.556

//     // self.new
// }

fn main() {
    rocket::ignite().mount("/hello", routes![hello]).launch();

    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let input: u32 = input.trim().parse().expect("Please input interger");

    let fahrenheit = Input {value: input};

    print!("fahrenheit = {}", fahrenheit.degree_converter());
}

