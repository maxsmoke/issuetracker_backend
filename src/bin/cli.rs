use issuetracker::db::{
    create_issue,
    query_issues,
    create_project,
    query_projects,
    establish_connection,
};
use std::env;

fn help() {
    println!("subcommands:");
    println!("      new_issue <title> <project_id>: create new issues.");
    println!("      new_proj <title>: create new Proj.");
    println!("      show_issues: show all issues.");
    println!("      show_proj: show all projects.");
    // println!("      delete <id>: delete task.");
    // println!("      mark_done <id>: update task done status.");
}

fn new_issue(args: &[String]) {
    if args.len() < 2 {
        println!("new_issue: missing <project_id>");
        help();
        return;
    }

    let to_convert = &args[1].clone();
    let num: i32;

    match to_convert.parse::<i32>() {
        Ok(n) => num = n,
        Err(_e) => panic!("Not a number!!"),
    };

    let conn = establish_connection();
    create_issue(&conn, &args[0], num);
}

fn new_project(args: &[String]){
    if args.len() < 1 {
        println!("new_proj: missing <title>");
        help();
        return;
    }
    let conn = establish_connection();
    create_project(&conn, &args[0]);
}

fn show_issues(args: &[String]) {
    if args.len() > 0 {
        println!("show_issues: unexpected argument.");
        help();
        return;
    }
    let conn = establish_connection();
    println!("Issues\n---");

    for issue in query_issues(&conn) {
        println!("Title: {} | Content: {} | Complete: {} | Project_ID: {}", 
        issue.title, 
        issue.content, 
        issue.complete,
        issue.project_id
    )
    }
}

//TODO: Add number of open issues in project
fn show_projects(args: &[String]){
    if args.len() > 0 {
        println!("show_proj: unexpected argument.");
        help();
        return;
    }
    let conn = establish_connection();
    println!("Projects\n---");

    for proj in query_projects(&conn){
        println!("Title: {} | Complete: {} ", 
        proj.title, 
        proj.complete,
    )
    }
}

fn main() {
    // cargo run --bin cli <command> <args>
    //  -3   -2   -1    0      1       2...
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        help();
        return;
    }

    let subcommand = &args[1];
    match subcommand.as_ref() {
        "new_issue" => new_issue(&args[2..]),
        "show_issues" => show_issues(&args[2..]),
        "new_proj" => new_project(&args[2..]),
        "show_projs" => show_projects(&args[2..]),
        // "mark_done" => mark_done(&args[2..]),
        // "delete" => delete(&args[2..]),
        _ => help(),
    }
}