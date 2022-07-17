extern crate core;

mod cli;
mod db;
mod err;
mod twitter;
mod utils;
mod version;

use chrono::Utc;
use std::env;
use std::process::exit;

use crate::err::Error;
use prettytable::{cell, row};
use serde::{Deserialize, Serialize};
use twitter::Credential;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Twitimer {
    id: u32,
    begin_at: chrono::DateTime<Utc>,
    begin_done: bool,
    tweet_id: Option<u64>,
    end_at: Option<chrono::DateTime<Utc>>,
    end_done: bool,
    draft: String,
    // note: String,
}

impl Twitimer {
    fn to_table_row(&self) -> prettytable::Row {
        let tweet_id_str = if self.tweet_id.is_none() {
            "/".to_string()
        } else {
            self.tweet_id.unwrap().to_string()
        };
        let end_at_str = if self.end_at.is_none() {
            "/".to_string()
        } else {
            self.end_at.unwrap().to_string()
        };
        row![
            self.id.to_string(),
            self.begin_at.to_string(),
            self.begin_done.to_string(),
            tweet_id_str,
            end_at_str,
            self.end_done.to_string(),
            self.draft.clone()
        ]
    }
}

#[derive(Clone, Debug)]
pub struct Config {
    version: version::Version,
    credential: Credential,
}

const PROGRAM_NAME: &str = "twitimer";
const PROGRAM_INIT_DESC: &str = "Init or update twitimer configuration";
const PROGRAM_NEW_DESC: &str = "Create a new twitimer task";
const PROGRAM_LIST_DESC: &str = "List your twitimer task(s)";
const PROGRAM_REMOVE_DESC: &str = "Remove your twitimer task(s)";
const PROGRAM_CRON_DESC: &str = "Check for actions";

const TWITIMER_VER: &str = "v0.0.1(beta)";

fn main() -> Result<(), err::Error> {
    let program_args: Vec<String> = env::args().collect();
    let program_args: Vec<&str> = program_args.iter().map(|s| s.as_str()).collect();

    // $ twitimer
    if program_args.len() < 2 {
        cli::print_help();
        exit(-1);
    }

    let sub = program_args[1];

    // $ twitimer xxx
    if !(sub.eq("help")
        || sub.eq("init")
        || sub.eq("new")
        || sub.eq("list")
        || sub.eq("remove")
        || sub.eq("cron"))
    {
        cli::print_help();
        exit(-1);
    }

    // $ twitimer help
    if program_args[1].eq("help") {
        cli::print_help();
        return Ok(());
    }

    let db_exist = db::exist();
    let db_available = db::check_available();

    if db_exist && !db_available {
        println!("db exists but not available!");
        println!("please remove it manually and use `$ twitimer init` to create a new one!");
        println!("db path: {}", db::db_path());
        exit(-2)
    }

    // $ twitimer init
    if program_args[1].eq("init") {
        let mut args = cli::init_args();
        args.parse(&program_args)
            .expect("Error when parsing program arguments");

        // $ twitimer init --help
        if args
            .value_of("help")
            .expect("Error when getting the value of flag help")
        {
            println!("{}", args.usage());
            return Ok(());
        }

        cli::init::handler(&args, db_exist).expect("Error when running `$ twitimer init`");
        return Ok(());
    }

    if !db_exist {
        println!("db not exist!");
        println!("execute `$ twitimer init` to create a new one!");
        exit(-1)
    }

    let conn = db::new_conn().expect("Error when establish connection to database");

    // $ twitimer new
    if program_args[1].eq("new") {
        let mut args = cli::new_args();
        args.parse(&program_args)
            .expect("Error when parsing program arguments");

        // $ twitimer new --help
        if args
            .value_of("help")
            .expect("Error when getting the value of flag help")
        {
            println!("{}", args.usage());
            return Ok(());
        }

        cli::new::handler(&conn, &args)?;
        return Ok(());
    }

    // $ twitimer list
    if program_args[1].eq("list") {
        let mut args = cli::list_args();
        args.parse(&program_args)
            .expect("Error when parsing program arguments");

        // $ twitimer list --help
        if args
            .value_of("help")
            .expect("Error when getting the value of flag help")
        {
            println!("{}", args.usage());
            return Ok(());
        }

        cli::list::handler(&conn, &args)?;
        return Ok(());
    }

    // $ twitimer remove
    if program_args[1].eq("remove") {
        let mut args = cli::remove_args();
        args.parse(&program_args)
            .expect("Error when parsing program arguments");

        // $ twitimer remove --help
        if args
            .value_of("help")
            .expect("Error when getting the value of flag help")
        {
            println!("{}", args.usage());
            return Ok(());
        }

        cli::remove::handler(&conn, &args)?;
        return Ok(());
    }

    // $ twitimer cron
    if program_args[1].eq("cron") {
        let mut args = cli::cron_args();
        args.parse(&program_args)
            .expect("Error when parsing program arguments");

        // $ twitimer cron --help
        if args
            .value_of("help")
            .expect("Error when getting the value of flag help")
        {
            println!("{}", args.usage());
            return Ok(());
        }

        cli::cron::handler(&conn, &args)?;
        return Ok(());
    }

    Ok(())
}
