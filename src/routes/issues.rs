use crate ::json::JsonIssueResponse;
use crate ::db::{ establish_connection, models };
use rocket::http::RawStr;
use rocket_contrib::json::Json;
use models::{
    Issue,
    InsertableIssue,
};
use rocket::http::Status;
use rocket::response::status;
use rocket::response;

#[get("/issues")]
pub fn get() -> Json<JsonIssueResponse>{
    let mut response = JsonIssueResponse{ data: vec![] };

    let conn = establish_connection();
    for issue in models::Issue::all(&conn) {
        response.data.push(issue);
    }
    Json(response)
}

#[get("/issue?<id>")]
pub fn get_issue(id: Option<&RawStr>) -> Json<JsonIssueResponse> {
    let conn = establish_connection();

    let raw_str = match id {
        Some(e) => e,
        None => RawStr::from_str("0"),
    };

    let int_id = match raw_str
        .url_decode()
        .unwrap()
        .parse::<i32>(){
            Ok(e) => e,
            Err(_e) => 0,
        };
    let result = models::Issue::get(int_id, &conn);
    Json( JsonIssueResponse{ data: vec![result]})
}
#[post("/issue/new", format="application/json", data="<issue>")]
pub fn new(issue: Json<InsertableIssue>){
    let conn = establish_connection();
    InsertableIssue::insert(issue.into_inner(), &conn);
}

//TODO Reroute to refreshed page
#[put("/issue/update/<id>", format="application/json", data="<issue>")]
pub fn update(id: i32, issue: Json<Issue>){
    let conn = establish_connection();
    Issue::update(id, issue.into_inner(), &conn);
}

//TODO Route to all issues
#[delete("/issue/delete/<id>")]
pub fn delete(id: i32) -> Status{
    let conn = establish_connection();
    Issue::delete(id, &conn)
} 