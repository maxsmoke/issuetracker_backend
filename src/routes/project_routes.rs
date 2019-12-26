use crate ::json::JsonProjectResponse;
use crate ::db::{ establish_connection, models };
use models::{
    Project,
    NewProject
};
use diesel::SqliteConnection;
use diesel::result::Error;
use rocket::http::{ RawStr, Status};
use rocket::response::{/* Failure, */ status};
use rocket_contrib::json::Json;


#[get("/projects")]
pub fn get_projects() -> Json<JsonProjectResponse>{
    let mut response = JsonProjectResponse{ data: vec![] };

    let conn = establish_connection();
    for project in models::Project::all(&conn) {
        response.data.push(project);
    }
    Json(response) 
}

#[get("/project?<id>")]
pub fn query_projects(id: Option<&RawStr>) -> Json<JsonProjectResponse> {

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
pub fn new_project(project: Json<NewProject>){
    //must contain all values in New Project
    let conn = establish_connection();
    NewProject::insert(project.into_inner(), &conn)
}


#[put("/project/update/<id>", format="application/json", data="<project>")]
pub fn update_project(id: i32, project: Json<Project>){
    let conn = establish_connection();
    Project::update(id, project.into_inner(), &conn);
}
/*
#[delete("")] */