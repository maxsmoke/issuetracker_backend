use super::schema::{issue, project};
use diesel;
use diesel::{prelude::*, sqlite::SqliteConnection, RunQueryDsl};
use crate::db::schema::issue::dsl::{
    issue as all_issues, project_id,
};
use crate::db::schema::project::dsl::{
    issue_count, project as all_projects,
};
use rocket::http::Status;


use super::schema::{ issue as issues, project as projects };

#[derive(Insertable, Deserialize)]
#[table_name = "issue"]
pub struct InsertableIssue {
    pub title: String,
    pub project_id: i32,
    pub complete: i32,
    pub content: String,
}
impl InsertableIssue{
    pub fn insert(issue: InsertableIssue, conn: &SqliteConnection) {
        diesel::insert_into(issue::table)
            .values(issue)
            .execute(conn)
            .expect("Error inserting new issue");
    }
}

#[derive(AsChangeset, Queryable, Deserialize, Serialize)]
#[table_name="issues"]
pub struct Issue {
    pub id: i32,
    pub title: String,
    pub project_id: i32,
    pub complete: i32,
    pub content: String,
}
impl Issue { 
    pub fn all(conn: &SqliteConnection) -> Vec<Issue>{
        issue::table
            .load::<Issue>(conn)
            .expect("Error loading Issues")
    }
    pub fn get(id: i32, conn: &SqliteConnection) ->  Issue {
        issue::table.find(id).get_result::<Issue>(conn).expect("Error: Failed Project query")
    }
    pub fn update(id: i32, issue: Issue, conn: &SqliteConnection) -> Status{
        match diesel::update(issues::table.find(id))
        .set(&issue)
        .execute(conn){
            Ok(_e) => Status::Ok,
            Err(_e) => Status::InternalServerError,
        }
    }
    pub fn delete(id: i32, conn: &SqliteConnection) -> Status{
        match diesel::delete(issues::table.find(id)).execute(conn){
            Ok(_e) => Status::Ok,
            Err(_e) => Status::NotFound,
        }
    }
}


#[derive(Insertable, Deserialize)]
#[table_name = "project"]
pub struct NewProject {
    pub title: String,
    pub complete: i32,
    pub issue_count: i32,
}
impl NewProject{
    pub fn insert(project: NewProject, conn: &SqliteConnection){
        diesel::insert_into(project::table)
            .values(project)
            .execute(conn)
            .expect("Error inserting new project");
    }
}

#[derive(AsChangeset, Debug, Deserialize, Serialize, Queryable)]
#[table_name="projects"]
pub struct Project {
    pub id: i32,
    pub title: String,
    pub complete: i32,
    pub issue_count: i32,
}
impl Project{
    pub fn all(conn: &SqliteConnection) -> Vec<Project>{
        let projects = project::table
        .load::<Project>(conn)
        .expect("Error loading Projects");

        print!("in all projects");
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

    pub fn get(conn: &SqliteConnection, id: i32) -> Project {
        project::table.find(id).get_result::<Project>(conn).expect("Error: Failed Project query")
    }
    pub fn update(id: i32, project: Project, conn: &SqliteConnection) -> Status{
        print!("{:?}", project);
        match diesel::update(projects::table.find(id))
        .set(&project)
        .execute(conn){
            Ok(_e) => Status::Ok,
            Err(_e) => Status::InternalServerError,
        }
    }
    pub fn delete(id: i32, conn: &SqliteConnection) -> Status{
        match diesel::delete(projects::table.find(id)).execute(conn){
            Ok(_e) => Status::Ok,
            Err(_e) => Status::NotFound,
        }
    }
}
