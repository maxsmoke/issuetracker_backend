/* #![feature(proc_macro_hygiene, decl_macro)]

// #[macro_use]
extern crate serde;

#[macro_use]
extern crate rocket;

extern crate rocket_contrib;

use db::{establish_connection, query_projects, query_issues};
use rocket_contrib::json::Json;

#[derive(Serialize)]
struct Json_ProjectResponse {
    data: Vec<models::Project>,
}

#[derive(Serialize)]
struct Json_IssueResponse {
    data: Vec<models::Issue>,
}

#[get("/")]
fn index() -> String {
    String::from("Hello World")
}

#[get("/projects")]
fn projects() -> Json<Json_ProjectResponse>{
    let mut response = Json_ProjectResponse{ data: vec![]};

    let conn = establish_connection();
    for project in query_projects(&conn) {
        response.data.push(project);
    }
    Json(response)
}

#[get("/issues")]
fn issues() -> Json<Json_IssueResponse>{
    let mut response = Json_IssueResponse{ data: vec![]};

    let conn = establish_connection();
    for issue in query_issues(&conn) {
        response.data.push(issue);
    }
    Json(response)
}

fn main() {
    // println!("Hello, world!");

    rocket::ignite().mount("/", routes![index]).launch();
}


 */


 
// #[macro_use]
// extern crate diesel;

// pub mod db;

fn main() {
    println!("Hello, world!");
}
/*
https://erwabook.com/intro/create-a-database-access-layer.html
*/