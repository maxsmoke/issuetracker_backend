use issuetracker::db::models;
use issuetracker::db::{
    close_item, establish_connection, 
};
use std::env;

fn help() {
    println!("subcommands:");
    println!("      new_issue <title> <project_id>: create new issues.");
    println!("      new_proj <title>: create new Proj.");
    println!("      show_issues: show all issues.");
    println!("      show_proj: show all projects.");
    println!("      issue_status <id>: update issue status.");
    println!("      proj_status <id>: update proj status.");
    // println!("      delete <id>: delete task.");
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

    models::NewIssue::create_issue(&conn, &args[0], num);
}

fn new_project(args: &[String]) {
    if args.len() < 1 {
        println!("new_proj: missing <title>");
        help();
        return;
    }
    let conn = establish_connection();

    models::NewProject::create_project(&conn, &args[0]);
}

fn show_issues(args: &[String]) {
    if args.len() > 0 {
        println!("show_issues: unexpected argument.");
        help();
        return;
    }
    let conn = establish_connection();
    println!("Issues\n---");

    for issue in models::Issue::show_issues(&conn) {
        println!(
            "Issue ID: {} | Title: {} | Content: {} | Complete: {} | Project_ID: {}",
            issue.id, issue.title, issue.content, issue.complete, issue.project_id
        )
    }
}
fn issue_done(args: &[String]) {
    if args.len() > 1 {
        println!("issue_status: too many args");
        help();
        return;
    }
    let conn = establish_connection();
    let conversion = &args[0].to_string();

    let _err = match conversion.parse::<i32>() {
        Ok(n) => close_item(&conn, &n, 1),
        Err(e) => panic!("id: not a number {}", e),
    };
}
fn proj_done(args: &[String]) {
    if args.len() > 1 {
        println!("issue_status: too many args");
        help();
        return;
    }
    let conn = establish_connection();
    let conversion = &args[0].to_string();

    let _err = match conversion.parse::<i32>() {
        Ok(n) => close_item(&conn, &n, 0),
        Err(e) => panic!("id: not a number {}", e),
    };
}

fn show_projects(args: &[String]) {
    if args.len() > 0 {
        println!("show_proj: unexpected argument.");
        help();
        return;
    }
    let conn = establish_connection();
    println!("Projects\n---");

    for proj in models::Project::show_projects(&conn){
        println!(
            "Title: {} | Complete: {} | No. of Issues: {} ",
            proj.title, proj.complete, proj.issue_count,
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
        "issue_status" => issue_done(&args[2..]),
        "proj_status" => proj_done(&args[2..]),
        // "mark_done" => mark_done(&args[2..]),
        // "delete" => delete(&args[2..]),
        _ => help(),
    }
}
