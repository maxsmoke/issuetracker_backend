use issuetracker::db::{
    create_issue,
    query_issues,
    establish_connection,
};
use std::env;

fn help() {
    println!("subcommands:");
    println!("      new_issue <title> <project_id>: create new issues.");
    println!("      show_issues: show all issues.");
    // println!("      delete <id>: delete task.");
    // println!("      mark_done <id>: update task done status.");
}

fn new_issue(args: &[String]) {
    if args.len() < 2 {
        println!("new_issue: missing <project_id>");
        help();
        return;
    }
    // let num = &args.parse::<i32>();
    let to_convert = &args[1].clone();
    let num: i32;

    match to_convert.parse::<i32>() {
        Ok(n) => num = n,
        Err(e) => panic!("Not a number!!"),
    };

    let conn = establish_connection();
    create_issue(&conn, &args[0], num);
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

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        help();
        return;
    }

    let subcommand = &args[1];
    match subcommand.as_ref() {
        "new_issue" => new_issue(&args[2..]),
        "show_issues" => show_issues(&args[2..]),
        // "mark_done" => mark_done(&args[2..]),
        // "delete" => delete(&args[2..]),
        _ => help(),
    }
}