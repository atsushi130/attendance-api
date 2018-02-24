
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

mod schema;
use schema::attendances::dsl::*;

use diesel::prelude::*;
use diesel::SqliteConnection;
use dotenv::dotenv;
use std::env;

mod data;
use data::{ AttendanceRepository, AttendanceEntity };

fn main() {

    let connection = establish_connection();
    let repository = AttendanceRepository::from(connection);
    let results = repository.getAttendances();

    for attendance in results {
        println!("id: {}", attendance.id);
        println!("user: {}", attendance.user);
        println!("check_at: {}", attendance.check_at);
        println!("attendance_type: {}", attendance.attendance_type);
    }
}

fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
