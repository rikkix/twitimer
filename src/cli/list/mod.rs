use crate::utils::id;
use crate::{db, err, Twitimer};
use prettytable::{cell, row};
use std::{fs, path};

pub fn handler(conn: &rusqlite::Connection, args: &args::Args) -> Result<(), err::Error> {
    let ids = args
        .optional_value_of::<String>("id")
        .expect("Error when getting value of program argument id");

    let as_json = args
        .value_of::<bool>("json")
        .expect("Error when getting value of program argument json");
    let filename = args
        .optional_value_of::<String>("output")
        .expect("Error when getting value of program argument output");

    let mut tasks: Vec<Twitimer> = Vec::new();
    if ids.is_some() {
        tasks = db::query::tasks(conn, &id::parse_list(ids.unwrap().as_str()))?;
    } else {
        tasks = db::query::all_tasks(conn)?;
    }

    if as_json || filename.is_some() {
        let json_value = if filename.is_some() {
            serde_json::to_string(&tasks)
        } else {
            serde_json::to_string_pretty(&tasks)
        };
        let json_value = json_value.expect("Error when stringify tasks");
        if filename.is_none() {
            println!("{}", json_value);
        } else {
            fs::write(
                path::Path::new(filename.unwrap().as_str()),
                json_value.as_bytes(),
            )
            .expect("Error when writing file");
        }
    } else {
        print(tasks);
    }

    Ok(())
}

pub fn print(tasks: Vec<Twitimer>) {
    let mut table = prettytable::Table::new();
    table.add_row(row![
        "ID",
        "Start At",
        "Start Done",
        "Tweet ID",
        "End At",
        "End Done",
        "Draft"
    ]);
    for task in tasks {
        table.add_row(task.to_table_row());
    }
    table.printstd();
}
