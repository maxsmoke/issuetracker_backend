#![feature(proc_macro_hygiene, decl_macro)]

extern crate serde;

#[macro_use]
extern crate rocket;

extern crate rocket_contrib;

use issuetracker::db::{establish_connection, models};
use issuetracker::json::{
    JsonIssueResponse,
    JsonProjectResponse,
};
// use issuetracker::routes;

use rocket::http::RawStr;
use rocket_contrib::json::Json;

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

    let conn = establish_connection();

    let raw_str = match id {
        Some(e) =>  e,
        None => RawStr::from_str("0"),
    };

    let int_id = match raw_str.url_decode().unwrap().parse::<i32>(){
        Ok(e) => e,
        Err(_e) => 0,
    };
    
    let result = models::Project::query_projects(&conn, int_id);

    Json( JsonProjectResponse{ data: vec![result] })
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


