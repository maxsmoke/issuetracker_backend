use crate ::json::JsonIssueResponse;
use crate ::db::{ establish_connection, models };
use rocket::http::RawStr;
use rocket_contrib::json::Json;

#[get("/issues")]
pub fn get_issues() -> Json<JsonIssueResponse>{
    let mut response = JsonIssueResponse{ data: vec![] };

    let conn = establish_connection();
    for issue in models::Issue::all(&conn) {
        response.data.push(issue);
    }
    Json(response)
}

// #[get("/issue/<id>")]
#[get("/issue?<id>")]
pub fn query_issues(id: Option<&RawStr>) -> Json<JsonIssueResponse> {
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
/* #[post("")]
#[put("")]
#[delete("")] */