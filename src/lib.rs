#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate rocket;

extern crate rocket_contrib;

pub mod db;
pub mod json;
pub mod routes;

pub fn rocket() -> rocket::Rocket{
    rocket::ignite().mount("/", routes![
        routes::project_routes::get_projects,
        routes::project_routes::query_projects,
        routes::issue_routes::get_issues,
        routes::issue_routes::query_issue,
        routes::index,
    ])
}