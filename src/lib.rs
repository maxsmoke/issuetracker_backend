#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel as D;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate rocket;

extern crate rocket_contrib;

use rocket_contrib::databases::diesel;

// #[database("sqlite_logs")]
// pub struct LogsDbConn(diesel::SqliteConnection);


pub mod db;
pub mod json;
pub mod routes;

pub fn rocket() -> rocket::Rocket{
    rocket::ignite()
    // .attach(LogsDbConn::fairing())
    .mount("/", routes![
        routes::projects::delete,
        routes::projects::get,
        routes::projects::get_project,
        routes::projects::new,
        routes::projects::update,
        routes::issues::get,
        routes::issues::get_issue,
        routes::issues::new,
        routes::issues::update,
        routes::index,
    ])
}