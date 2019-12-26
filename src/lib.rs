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
        routes::project_routes::get_projects,
        routes::project_routes::new_project,
        routes::project_routes::update_project,
        routes::project_routes::query_projects,
        routes::project_routes::delete_project,
        routes::issue_routes::get_issues,
        routes::issue_routes::query_issues,
        routes::index,
    ])
}