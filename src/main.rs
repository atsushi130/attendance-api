
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

fn main() {

    let connection = establish_connection();
    let results = attendances
        .load::<AttendanceEntity>(&connection)
        .expect("Error loading attendances");

    println!("Displaying {} attendances", results.len());
    for attendance in results {
        println!("id: {}", attendance.id);
        println!("user: {}", attendance.user);
        println!("check_at: {}", attendance.check_at);
        println!("attendance_type: {}", attendance.attendance_type);
    }
}

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[derive(Queryable)]
pub struct AttendanceEntity {
    pub id: i32,
    pub user: String,
    pub check_at: String,
    pub attendance_type: i32
}
