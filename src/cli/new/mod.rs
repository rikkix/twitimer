use crate::utils::stdiotools;
use crate::utils::stdiotools::{ask_correct, yes_or_no};
use crate::utils::time::parse_date_time;
use crate::utils::time_cli::ask_for_date_time;
use crate::{cli, db, err, Twitimer};
use chrono::Utc;
use std::io::{stdin, Read};

const TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S %:z";

pub fn handler(conn: &rusqlite::Connection, args: &args::Args) -> Result<(), err::Error> {
    let silent = args
        .value_of("silent")
        .expect("Error when getting value of program argument silent");

    let mut draft = String::new();

    let msg: Option<String> = args
        .optional_value_of("message")
        .expect("Error when getting value of program argument message");
    if msg.is_some() {
        draft = msg.unwrap();
    } else {
        println!("Please enter your message below, ^+C (or Ctrl+C) to exit,");
        println!("newline and then press ^D (or Ctrl+D) to finish:");
        println!("==================================================");
        let mut buf: Vec<u8> = Vec::new();
        stdin()
            .lock()
            .read_to_end(&mut buf)
            .expect("Error when reading from stdin");
        println!("\n==================================================");
        draft = String::from_utf8(buf).expect("Error when treating the message as UTF-8");
        draft = draft.trim().to_string();
    }
    println!("Your Message:\n{}\n", draft);
    let info = twitter_text::parse(
        draft.as_str(),
        &twitter_text_config::Configuration::default(),
        true,
    );

    if !info.is_valid {
        return Err(err::Error::new(
            Some(3),
            "Then tweet draft is not valid!".to_string(),
        ));
    }

    println!("Words Count: {}/280", info.weighted_length);
    let correct = stdiotools::yes_or_no("Is is correct?", Some(true), silent);
    if !correct {
        println!("==================================================");
        return handler(conn, args);
    }

    let (start, end) = start_end_cli(args, silent)?;

    let task = Twitimer {
        // auto increase (default as 0)
        id: 0,
        begin_at: start,
        // default as false
        begin_done: false,
        // default as None
        tweet_id: None,
        end_at: end,
        // default as false
        end_done: false,
        draft,
    };

    db::insert::task(conn, &task)?;

    Ok(())
}

fn start_end_cli(
    args: &args::Args,
    silent: bool,
) -> Result<(chrono::DateTime<Utc>, Option<chrono::DateTime<Utc>>), err::Error> {
    let start: chrono::DateTime<Utc>;
    let end: Option<chrono::DateTime<Utc>>;
    let args_start: Option<String> = args
        .optional_value_of("start")
        .expect("Error when getting value of program argument start");

    let args_end: Option<String> = args
        .optional_value_of("start")
        .expect("Error when getting value of program argument start");

    if args_start.is_some() {
        let result = parse_date_time(&args_start.unwrap())?;
        start = result;
    } else {
        println!("Please enter the start time of your tweet below");
        start = ask_for_date_time();
    }

    if args_end.is_some() {
        let args_end = args_end.unwrap();
        if args_end.eq_ignore_ascii_case("none") {
            end = None;
        } else {
            let result = parse_date_time(&args_end)?;
            end = Some(result);
        }
    } else {
        if !yes_or_no("Do you want to set an end time?", Some(false), silent) {
            end = None;
        } else {
            println!("Please enter the end time of your tweet below");
            end = Some(ask_for_date_time());
        }
    }

    println!("Start time set to: {}", start.format(TIME_FORMAT));

    let end_str = if end.is_some() {
        end.clone().unwrap().format(TIME_FORMAT).to_string()
    } else {
        "None".to_string()
    };
    println!("End time set to: {}", end_str);

    if !yes_or_no("Are they correct?", Some(true), silent) {
        return start_end_cli(args, silent);
    }
    Ok((start, end))
}
