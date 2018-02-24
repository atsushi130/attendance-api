
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
use schema::attendance::dsl::*;

use diesel::prelude::*;
use diesel::SqliteConnection;
use dotenv::dotenv;
use std::env;

fn main() {
    println!("Hello, world!");

    let connection = establish_connection();
    let results = attendance
        .load::<Attendance>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("id: {}", post.id);
        println!("user: {}", post.user);
        println!("check_at: {}", post.check_at);
        println!("attendance_type: {}", post.attendance_type);
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
pub struct Attendance {
    pub id: i32,
    pub user: String,
    pub check_at: String,
    pub attendance_type: i32
}
