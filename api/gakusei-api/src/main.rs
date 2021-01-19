#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Try opening http://localhost:8000/yu :)"
}

#[get("/yu")]
fn yu() -> &'static str {
    "Hi Yu, ready to work?"
}

fn main() {
    rocket::ignite().mount("/", routes![index, yu]).launch();
}
