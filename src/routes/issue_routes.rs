use crate ::json::JsonIssueResponse;
use crate ::db::{ establish_connection, models };
// use rocket::http::RawStr;
use rocket_contrib::json::Json;

#[get("/issues")]
pub fn get_issues() -> Json<JsonIssueResponse>{
    let mut response = JsonIssueResponse{ data: vec![] };

    let conn = establish_connection();
    for issue in models::Issue::show_issues(&conn) {
        response.data.push(issue);
    }
    Json(response)
}