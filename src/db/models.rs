use super::schema::{issue, project};
// use crate::db::schema::issue::dsl::{
    // complete as new_complete_issue, issue as all_issues, project_id,
// };
use crate::diesel::RunQueryDsl;
use diesel::SqliteConnection;

#[derive(Insertable)]
#[table_name = "issue"]
pub struct NewIssue<'a> {
    pub title: &'a str,
    pub project_id: i32,
    pub complete: i32,
    pub content: String,
}
impl NewIssue<'_> {
    pub fn create_issue<'a>(conn: &SqliteConnection, title: &String, id: i32) {
       let issue = NewIssue {
           title,
           project_id: id,
           complete: 0,
           content: String::from(""),
       };
        diesel::insert_into(issue::table)
            .values(issue)
            .execute(conn)
            .expect("Error inserting new issue");
    }
}

#[derive(Queryable, Serialize)]
pub struct Issue {
    pub id: i32,
    pub title: String,
    pub project_id: i32,
    pub complete: i32,
    pub content: String,
}

#[derive(Insertable)]
#[table_name = "project"]
pub struct NewProject<'a> {
    pub title: &'a str,
    pub complete: i32,
    pub issue_count: i32,
}
impl NewProject<'_>{
    pub fn create_project(conn: &SqliteConnection, title: &String){
        let project = NewProject {
                title,
                complete: 0,
                issue_count: 0,
            };
        
        diesel::insert_into(project::table)
            .values(project)
            .execute(conn)
            .expect("Error inserting new project");
    }
}

#[derive(Queryable, Serialize)]
pub struct Project {
    pub id: i32,
    pub title: String,
    pub complete: i32,
    pub issue_count: i32,
}
