use diesel;
use diesel::{prelude::*, sqlite::SqliteConnection};

use crate::db::schema::issue::dsl::{
    // complete as complete_issues, 
    issue as all_issues,
    project_id,
}; 

use crate::db::schema::project::dsl::{
    project as all_projects,
    issue_count,
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

pub fn create_project<'a>(conn: &SqliteConnection, title: &'a str){
    let project = models::NewProject {
        title: title,
        complete: 0,
    };
    diesel::insert_into(schema::project::table)
        .values(&project)
        .execute(conn)
        .expect("Error creating new project");
}
pub fn query_projects(conn: &SqliteConnection) -> Vec<models::Project>{
    // count_issues_in_projects(conn, 1);
    let projects = schema::project::table
    .load::<models::Project>(conn)
    .expect("Error loading Projects");

    //update issue count
    for project in projects{
        let new_count = count_issues_in_projects(conn, project.id);

        if project.issue_count != new_count{
            let update_project = diesel::update(all_projects.find(project.id));
        
            let result = update_project.set(issue_count.eq(new_count)).execute(conn);
    
            match result{
                Ok(e) => println!("{}", e),
                Err(_e) => panic!("update issue_count Failed"),
            };
        }
        
    };

    schema::project::table
    .load::<models::Project>(conn)
    .expect("Error loading Projects")
}

//used to update how many projects
fn count_issues_in_projects(conn: &SqliteConnection, id: i32) -> i32 {
    let query = all_issues.find(id).count().get_result::<i64>(conn);
    let conversion = query.unwrap() as i32;
    
    conversion
}
/* fn update_project(conn: &SqliteConnection){

} */
