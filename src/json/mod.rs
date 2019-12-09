use super::db::models;

#[derive(Serialize)]
pub struct JsonProjectResponse {
    pub data: Vec<models::Project>,
} 

#[derive(Serialize)]
pub struct JsonIssueResponse {
    pub data: Vec<models::Issue>,
}