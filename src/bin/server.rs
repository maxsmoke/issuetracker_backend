#![feature(proc_macro_hygiene, decl_macro)]

// #[macro_use]
extern crate serde;

#[macro_use]
extern crate rocket;

extern crate rocket_contrib;

use issuetracker::db::{establish_connection, /* models, */ query_projects, query_issues};
use rocket_contrib::json::Json;
use issuetracker::json::{
    JsonIssueResponse,
    JsonProjectResponse,
};

/* #[derive(Serialize)]
struct JsonProjectResponse {
    data: Vec<models::Project>,
} 

#[derive(Serialize)]
struct JsonIssueResponse {
    data: Vec<models::Issue>,
} */

#[get("/")]
fn index() -> String {
    String::from("Hello World")
}

#[get("/projects")]
fn get_projects() -> Json<JsonProjectResponse>{
    let mut response = JsonProjectResponse{ data: vec![]};

    let conn = establish_connection();
    for project in query_projects(&conn) {
        response.data.push(project);
    }
    Json(response)
}

#[get("/issues")]
fn get_issues() -> Json<JsonIssueResponse>{
    let mut response = JsonIssueResponse{ data: vec![]};

    let conn = establish_connection();
    for issue in query_issues(&conn) {
        response.data.push(issue);
    }
    Json(response)
}

fn main() {
    rocket::ignite().mount("/", routes![index, get_issues,get_projects]).launch();
}


