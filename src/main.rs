mod cli;
mod db;
mod err;
mod utils;
mod version;

use chrono::{Local, Utc};
use std::process::exit;
use std::time::Instant;
use std::{env, io, path, result};

use crate::db::DB_PATH;
use crate::err::Error;
use egg_mode::tweet;

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct Credential {
    consumer_key: String,
    consumer_secret: String,
    access_key: String,
    access_secret: String,
}

#[derive(Clone, Debug)]
pub struct Config {
    version: version::Version,
    credential: Credential,
}

impl Credential {
    fn check_empty(&self) -> Result<(), Error> {
        if self.consumer_key.is_empty() {
            return Err(Error::new(None, "consumer_key is empty".to_string()));
        }
        if self.consumer_secret.is_empty() {
            return Err(Error::new(None, "consumer_secret is empty".to_string()));
        }
        if self.access_key.is_empty() {
            return Err(Error::new(None, "access_key is empty".to_string()));
        }
        if self.access_secret.is_empty() {
            return Err(Error::new(None, "access_secret is empty".to_string()));
        }
        Ok(())
    }
}

#[derive(Clone)]
pub struct CredentialOpt {
    consumer_key: Option<String>,
    consumer_secret: Option<String>,
    access_key: Option<String>,
    access_secret: Option<String>,
}

impl CredentialOpt {
    fn to_credential(self) -> Result<Credential, Error> {
        if self.consumer_key.is_none() {
            return Err(Error::new(
                None,
                "Unable to convert CredentialOpt to Credential: consumer_key is none".to_string(),
            ));
        }
        if self.consumer_secret.is_none() {
            return Err(Error::new(
                None,
                "Unable to convert CredentialOpt to Credential: consumer_secret is none"
                    .to_string(),
            ));
        }
        if self.access_key.is_none() {
            return Err(Error::new(
                None,
                "Unable to convert CredentialOpt to Credential: access_key is none".to_string(),
            ));
        }
        if self.access_secret.is_none() {
            return Err(Error::new(
                None,
                "Unable to convert CredentialOpt to Credential: access_secret is none".to_string(),
            ));
        }

        Ok(Credential {
            consumer_key: self.consumer_key.unwrap(),
            consumer_secret: self.consumer_secret.unwrap(),
            access_key: self.access_key.unwrap(),
            access_secret: self.access_secret.unwrap(),
        })
    }
}

const PROGRAM_NAME: &str = "twitimer";
const PROGRAM_INIT_DESC: &str = "Init or update twitimer configuration";
const PROGRAM_NEW_DESC: &str = "Create a new twitimer task";
const PROGRAM_LIST_DESC: &str = "List your twitimer task(s)";
const PROGRAM_EDIT_DESC: &str = "Edit your twitimer task";
const PROGRAM_CRON_DESC: &str = "Check for actions";

const TWITIMER_VER: &str = "v0.0.1(dev)";

fn main() -> io::Result<()> {
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
        || sub.eq("edit")
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
        println!("db path: {}", db::DB_PATH);
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

        // $twitimer init --update
        if args
            .value_of("update")
            .expect("Error when getting the value of flag update")
        {
            if !db_exist {
                println!("No database found at {}", DB_PATH);
                println!("Use `$ twitimer init` to initialize one");
                exit(-3)
            }

            // TODO
            return Ok(());
        }

        if db_exist {
            println!("database exists!");
            println!("use `$ twitimer init --update` to upgrade configurations!");
            exit(-1)
        }
        cli::init::init_db_cli(&args).expect("Error when running `$ twitimer init`");
        return Ok(());
    }

    // $ twitimer new
    if program_args[1].eq("new") {
        return Ok(());
    }

    // $ twitimer list
    if program_args[1].eq("list") {
        return Ok(());
    }

    // $ twitimer edit
    if program_args[1].eq("edit") {
        return Ok(());
    }

    // $ twitimer cron
    if program_args[1].eq("cron") {
        return Ok(());
    }

    Ok(())
}
