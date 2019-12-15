pub mod project_routes;
pub mod issue_routes;

#[get("/")]
pub fn index() -> String {
    String::from("Hello World")
} 