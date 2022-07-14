use crate::utils::id;
use crate::utils::stdiotools::yes_or_no;
use crate::{cli, db, err};
use std::process::exit;

pub fn handler(conn: &rusqlite::Connection, args: &args::Args) -> Result<(), err::Error> {
    let ignore = args
        .value_of::<bool>("ignore")
        .expect("Error when getting value of program argument silent");

    let ids = args
        .optional_value_of::<String>("id")
        .expect("Error when getting value of program argument id");

    if ids.is_none() {
        return Err(err::Error::new(
            None,
            "Must pass in --id ID as argument".to_string(),
        ));
    }

    let ids = id::parse_list(ids.unwrap().as_str());

    if !ignore {
        println!("The tasks to be removed:");
        cli::list::print(db::query::tasks(conn, &ids)?);
        println!();
        if !yes_or_no("Are the correct", Some(false), false) {
            println!("User cancelled");
            exit(1);
        }
    }

    db::remove::tasks(conn, &ids)?;
    Ok(())
}
