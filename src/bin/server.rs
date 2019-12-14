#![feature(proc_macro_hygiene, decl_macro)]

// #[macro_use]
extern crate serde;

#[macro_use]
extern crate rocket;

extern crate rocket_contrib;

use issuetracker::db::{establish_connection, models};
use rocket::http::RawStr;
use rocket_contrib::json::Json;
use issuetracker::json::{
    JsonIssueResponse,
    JsonProjectResponse,
};

#[get("/")]
fn index() -> String {
    String::from("Hello World")
}

#[get("/projects")]
fn get_projects() -> Json<JsonProjectResponse>{
    let mut response = JsonProjectResponse{ data: vec![] };

    let conn = establish_connection();
    for project in models::Project::show_projects(&conn) {
        response.data.push(project);
    }
    Json(response)
}

// #[get("/project/<id>")]
#[get("/project?<id>")]
fn query_projects(id: Option<&RawStr>) -> Json<JsonProjectResponse> {
    let raw_str = id.unwrap();
    let decode = raw_str.url_decode().unwrap();
    let int_id = decode.parse::<i32>().unwrap();
    
    let mut response = JsonProjectResponse{ data: vec![] };

    let conn = establish_connection();

    let result = models::Project::query_projects(&conn, int_id);

    response.data.push(result);
    Json(response)
}

#[get("/issues")]
fn get_issues() -> Json<JsonIssueResponse>{
    let mut response = JsonIssueResponse{ data: vec![] };

    let conn = establish_connection();
    for issue in models::Issue::show_issues(&conn) {
        response.data.push(issue);
    }
    Json(response)
}

fn main() {
    rocket::ignite().mount("/", routes![index, get_issues, get_projects, query_projects]).launch();
}


