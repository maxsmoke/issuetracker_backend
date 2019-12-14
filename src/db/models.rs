use super::schema::{issue, project};
use diesel;
use diesel::{prelude::*, sqlite::SqliteConnection};
use crate::db::schema::issue::dsl::{
    complete as new_complete_issue, issue as all_issues, project_id,
};
use crate::db::schema::project::dsl::{
    complete as new_complete_proj, issue_count, project as all_projects,
};
use crate::diesel::RunQueryDsl;
// use diesel::SqliteConnection;

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
        diesel::insert_into(issue::table)
            .values(
                NewIssue{
                    title,
                    project_id: id,
                    complete: 0, 
                    content: String::from("")
                })
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
impl Issue { 
    pub fn show_issues(conn: &SqliteConnection) -> Vec<Issue>{
        issue::table
            .load::<Issue>(conn)
            .expect("Error loading Issues")
    }
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
        diesel::insert_into(project::table)
            .values(
                NewProject {
                title,
                complete: 0,
                issue_count: 0,
            })
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
impl Project{
    pub fn show_projects(conn: &SqliteConnection) -> Vec<Project>{
        let projects = project::table
        .load::<Project>(conn)
        .expect("Error loading Projects");

    //update issue count
    for project in projects {
        let new_count = all_issues
            .filter(project_id.eq(project.id))
            .count()
            .get_result::<i64>(conn)
            .unwrap() as i32;

        if project.issue_count != new_count {
            let update_project = diesel::update(all_projects.find(project.id));

            let result = update_project.set(issue_count.eq(new_count)).execute(conn);

            match result {
                Ok(e) => println!("{}", e),
                Err(_e) => panic!("update issue_count Failed"),
            };
        }
    }
        project::table
            .load::<Project>(conn)
            .expect("Error loading Projects")
    }
}
