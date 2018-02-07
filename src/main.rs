#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] 
extern crate failure;
extern crate rocket;
extern crate rocket_contrib;

use rocket::Request;
use failure::Error;
use rocket_contrib::Template;
use std::collections::HashMap;

#[derive(Debug, Fail)]
enum WebTransError {
    #[fail(display = "invalid toolchain name: {}", name)]
    InvalidToolchainName {
        name: String,
    }
}


#[get("/")]
fn index() -> Template {
    let mut map: HashMap<String, String> = HashMap::new(); 
    Template::render("index", map)
}


#[error(404)]
fn not_found(req: &Request) -> &'static str {
    "error"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .attach(Template::fairing())
        .catch(errors![not_found])
        .launch();
}