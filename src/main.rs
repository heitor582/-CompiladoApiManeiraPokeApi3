#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
extern crate mongodb;
use rocket_cors::Cors;

fn cors_fairing() -> Cors {
    Cors::from_options(&Default::default()).expect("Cors fairing cannot be created")
}

mod controllers;
mod lib;
mod meta;
mod models;
mod db;

fn main() {
    rocket::ignite()
    .mount("/", routes![])
    .register(catchers![])
    .attach(cors_fairing())
    .launch();
}
