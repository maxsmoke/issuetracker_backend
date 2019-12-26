pub mod projects;
pub mod issues;

#[get("/")]
pub fn index() -> String {
    String::from("Hello World")
} 