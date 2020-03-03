use crate ::json::JsonProjectResponse;
use crate ::db::{ establish_connection, models };
use models::{
    Project,
    NewProject
};
use rocket::http::RawStr;
use rocket_contrib::json::Json;
// use rocket::response::status;
use rocket::http::Status;
use rocket::response::{Redirect, status};
use rocket::response;


#[get("/projects")]
pub fn get() -> Json<JsonProjectResponse> {
    let mut response = JsonProjectResponse{ data: vec![] };

    let conn = establish_connection();
    for project in models::Project::all(&conn) {
        response.data.push(project);
    }
    Json(response) 
}

#[get("/project/<id>")]
pub fn get_project(id: Option<&RawStr>) -> Json<JsonProjectResponse> {

    let conn = establish_connection();

    let raw_str = match id {
        Some(e) =>  e,
        None => RawStr::from_str("0"),
    };

    let int_id = match raw_str
        .url_decode()
        .unwrap()
        .parse::<i32>(){
            Ok(e) => e,
            Err(_e) => 0,
        };
    
    let result = models::Project::get(&conn, int_id);

    Json( JsonProjectResponse{ data: vec![result]})
}

#[post("/project/new", format="application/json", data="<project>")]
pub fn new(project: Json<NewProject>){
    // must contain all values in New Project
    // figure out validation on frontend
    let conn = establish_connection();
    NewProject::insert(project.into_inner(), &conn)
}

//TODO Reroute to refreshed page
// Kind of works but not redirecting
#[put("/project/update/<id>", format="application/json", data="<project>")]
pub fn update(id: i32, project: Json<Project>) /* -> Redirect */{
    let conn = establish_connection();
    Project::update(id, project.into_inner(), &conn);
    // let convert = id.to_string();
    // Redirect::to(uri!(get_project: RawStr::from_str(&convert)))
}
//TODO Route to all projects
#[delete("/project/delete/<id>")]
pub fn delete(id: i32) -> Status{
    let conn = establish_connection();
    Project::delete(id, &conn)
}