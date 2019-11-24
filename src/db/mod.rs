use diesel;
use diesel::{prelude::*, sqlite::SqliteConnection};

use crate::db::schema::issue::dsl::{
    complete as complete_issues, 
    // issues as all_issues
};

pub mod models;
pub mod schema;

pub fn establish_connection() -> SqliteConnection {
    let db = "./issuetracker.sqlite3";
    SqliteConnection::establish(db).unwrap_or_else(|_| panic!("Error connecting to {}", db))
}

pub fn create_issue<'a>(conn: &SqliteConnection, title: &'a str, proj: i32) {
    let issue = models::NewIssue { 
        title: title, 
        project_id: proj, 
        complete: 0, 
        content: String::from("")
    };
    diesel::insert_into(schema::issue::table)
        .values(&issue)
        .execute(conn)
        .expect("Error inserting new issue");
}

pub fn query_issues(conn: &SqliteConnection) -> Vec<models::Issue> {
    schema::issue::table
        .load::<models::Issue>(conn)
        .expect("Error loading Issues")
}

