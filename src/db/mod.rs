use diesel;
use diesel::{prelude::*, sqlite::SqliteConnection};

use crate::db::schema::issue::dsl::{
    complete as new_complete_issue, issue as all_issues,
};

use crate::db::schema::project::dsl::{
    complete as new_complete_proj, project as all_projects,
};

pub mod models;
pub mod schema;

pub fn establish_connection() -> SqliteConnection {
    let db = "./issuetracker.sqlite3";
    SqliteConnection::establish(db).unwrap_or_else(|_| panic!("Error connecting to {}", db))
}

//used for CLI
pub fn close_item(conn: &SqliteConnection, id: &i32, type_change: i8) -> QueryResult<usize> {
    let issue_target;
    let proj_target;

    if type_change == 1 {
        // issues
        issue_target = all_issues
            .find(id)
            .get_result::<models::Issue>(conn)
            .unwrap();
        let new_status = match issue_target.complete {
            1 => 0,
            0 => 1,
            _ => 0,
        };

        let update_issue = diesel::update(all_issues.find(id));
        update_issue
            .set(new_complete_issue.eq(new_status))
            .execute(conn)
    } else {
        // projects
        proj_target = all_projects
            .find(id)
            .get_result::<models::Project>(conn)
            .unwrap();
        let new_status = match proj_target.complete {
            1 => 0,
            0 => 1,
            _ => 0,
        };

        let update_proj = diesel::update(all_projects.find(id));
        update_proj
            .set(new_complete_proj.eq(new_status))
            .execute(conn)
    }
}

/* fn update_project(conn: &SqliteConnection){

} */
