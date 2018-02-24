
#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;
extern crate serde;

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;

mod schema;

use diesel::prelude::*;
use diesel::SqliteConnection;
use dotenv::dotenv;
use std::env;

mod data;
mod presentation;
mod database;

fn main() {

    presentation::routes();

    /*
    let connection = establish_connection();
    let repository = AttendanceRepository::from(connection);
    let results = repository.getAttendances();

    for attendance in results {
        println!("id: {}", attendance.id);
        println!("user: {}", attendance.user);
        println!("check_at: {}", attendance.check_at);
        println!("attendance_type: {}", attendance.attendance_type);
    }*/
}
