use diesel;
use diesel::{prelude::*, sqlite::SqliteConnection};

use crate::db::schema::issue::dsl::{
    // complete as complete_issues, 
    issue as all_issues,
    complete as new_complete_issue,
    // project_id,
}; 

use crate::db::schema::project::dsl::{
    project as all_projects,
    complete as new_complete_proj,
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
pub fn close_item(conn: &SqliteConnection, id: &i32, type_change: i8) -> QueryResult<usize> {
    let issue_target;
    let proj_target;
    if type_change == 1 { // issues

        issue_target = all_issues.find(id).get_result::<models::Issue>(conn).unwrap();
        let new_status = match issue_target.complete {
            1 => 0,
            0 => 1,
            _ => 0,
        };

        let update_issue = diesel::update(all_issues.find(id));
        update_issue.set(new_complete_issue.eq(new_status)).execute(conn)
        
        // println!("Issue status changed to: {}", new_status);
    } else { // projects

        proj_target = all_projects.find(id).get_result::<models::Project>(conn).unwrap();
        let new_status = match proj_target.complete {
            1 => 0,
            0 => 1,
            _ => 0,
        };

        let update_proj = diesel::update(all_projects.find(id));
        update_proj.set(new_complete_proj.eq(new_status)).execute(conn)
        // println!("Project status changed to: {}", new_status);
    }

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
        issue_count: 0,
    };
    diesel::insert_into(schema::project::table)
        .values(&project)
        .execute(conn)
        .expect("Error creating new project");
}
pub fn query_projects(conn: &SqliteConnection) -> Vec<models::Project>{
    count_issues_in_projects(conn, 1);
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
//TODO NOT QUERYING CORRECTLY finding all issues with same id. Which would always return one.
fn count_issues_in_projects(conn: &SqliteConnection, id: i32) -> i32 {
    let result = all_issues.find(id)
        .count()
        .get_result::<i64>(conn)
        .unwrap() as i32;

    println!("issue count {}", result);
    result

}
/* fn update_project(conn: &SqliteConnection){

} */
