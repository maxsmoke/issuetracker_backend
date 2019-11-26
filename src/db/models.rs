use super::schema::{issue, project};

#[derive(Insertable)]
#[table_name = "issue"]
pub struct NewIssue<'a> {
    pub title: &'a str,
    pub project_id: i32,
    pub complete: i32,
    pub content: String,
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
}

#[derive(Queryable, Serialize)]
pub struct Project {
    pub id: i32,
    pub title: String,
    pub complete: i32,
    pub issue_count: i32,
}