use crate ::json::JsonProjectResponse;
use crate ::db::{ establish_connection, models };
use diesel::SqliteConnection;
use rocket::http::RawStr;
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

// #[get("/project/<id>")]
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

/* #[post("/project/new/", format="application/json", data="<project>")]
pub fn create_project(title: Json<Project>, conn: SqliteConnection) {
    
   
}

#[put("")]
#[delete("")] */