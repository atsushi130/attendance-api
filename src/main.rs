
#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate diesel;

extern crate rocket;
extern crate serde;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate chrono;
extern crate reduce;

mod schema;
mod data;
mod domain;
mod presentation;
mod database;
mod extension;

fn main() {
    presentation::routes();
}
