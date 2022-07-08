use crate::{db, err};

pub fn handler(conn: &rusqlite::Connection, args: &args::Args) -> Result<(), err::Error> {
    let remove = args
        .value_of("remove")
        .expect("Error when getting the value of flag remove");
    let cred = db::query::config(conn)?.credential;
    println!("Starting performing send...");
    for task in db::query::tasks_to_start(conn)? {
        let result = cred.post(task.draft);
        match result {
            Ok(tweet_id) => {
                db::update::start_done(conn, task.id)?;
                db::update::tweet_id(conn, task.id, tweet_id)?
            }
            Err(e) => println!(
                "Error when performing send for task {}: {}\nSkipping...",
                task.id,
                err::Error::from(e).to_string()
            ),
        }
    }
    println!();

    println!("Starting performing delete...");
    for task in db::query::tasks_to_end(conn)? {
        let result = cred.delete(task.tweet_id.unwrap());
        if result.is_err() {
            println!(
                "Error when performing send for task {}: {}\nSkipping...",
                task.id,
                err::Error::from(result.unwrap_err()).to_string()
            );
            continue;
        }
        db::update::end_done(conn, task.id)?;
        if remove {
            db::remove::task(conn, task.id)?;
        }
    }

    Ok(())
}
