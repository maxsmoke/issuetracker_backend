use crate ::json::JsonIssueResponse;
use crate ::db::{ establish_connection, models };
use rocket::http::RawStr;
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

#[get("/issue?<id>")]
pub fn query_issue(id: Option<&RawStr>) -> Json<JsonIssueResponse> {
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
    let result = models::Issue::query_issues(&conn, int_id);
    Json( JsonIssueResponse{ data: vec![result]})
}