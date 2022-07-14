use crate::twitter::CredentialOpt;
use crate::utils::stdiotools::{ask_correct, clear_terminal, yes_or_no};
use crate::{db, version, Config, Error, TWITIMER_VER};
use rusqlite;
use scopeguard::defer;
use std::io;

pub fn handler(args: &args::Args, exist: bool) -> Result<(), Error> {
    let silent = args
        .value_of("silent")
        .expect("Error when getting value of program argument silent");

    let mut cred_opt = CredentialOpt {
        consumer_key: args
            .optional_value_of("consumer-key")
            .expect("Error when getting value of program argument consumer-key"),
        consumer_secret: args
            .optional_value_of("consumer-secret")
            .expect("Error when getting value of program argument consumer-secret"),
        access_key: args
            .optional_value_of("access-key")
            .expect("Error when getting value of program argument access-key"),
        access_secret: args
            .optional_value_of("access-secret")
            .expect("Error when getting value of program argument access-secret"),
    };

    cred_opt.consumer_key = Some(ask_correct("Consumer Key", cred_opt.consumer_key, silent));
    println!();
    cred_opt.consumer_secret = Some(ask_correct(
        "Consumer Secret",
        cred_opt.consumer_secret,
        silent,
    ));
    println!();
    cred_opt.access_key = Some(ask_correct("Access Key", cred_opt.access_key, silent));
    println!();
    cred_opt.access_secret = Some(ask_correct("Access Secret", cred_opt.access_secret, silent));
    println!();

    let cred = cred_opt.to_credential()?;

    println!("{}:    {},", "Consumer Key", cred.consumer_secret);
    println!("{}: {},", "Consumer Secret", cred.consumer_secret);
    println!("{}:      {},", "Access Key", cred.access_key);
    println!("{}:   {},", "Access Secret", cred.access_secret);
    if !yes_or_no("Are they correct?", Some(true), silent) {
        clear_terminal();
        println!("Please re-execute twitimer to reconfigure the credential!");
    }
    clear_terminal();
    println!("Great! Please wait while preparing the database...");

    let conn = db::new_conn().expect("Error when creating a new database");
    if !exist {
        db::init::structure(&conn).expect("Error when initialize database structure");
    }

    let conf = Config {
        version: version::Version::from(TWITIMER_VER).expect("Error when parsing TWITIMER_VER"),
        credential: cred,
    };

    db::insert::config(&conn, &conf).expect("Error when writing or updating config to database");
    println!("Successfully initialized or updated!");
    println!("Please re-execute the program to add/list/remove tasks.");

    Ok(())
}
