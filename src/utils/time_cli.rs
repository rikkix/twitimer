use crate::utils::time::parse_date_time;
use chrono::Utc;
use std::io::{stdin, stdout, BufRead, Write};

pub fn ask_for_date_time() -> chrono::DateTime<Utc> {
    print!("(Format: 2022-02-12 13:23:45 +9): ");
    stdout().lock().flush().expect("Error when flushing stdout");
    let mut buf = String::new();
    stdin()
        .lock()
        .read_line(&mut buf)
        .expect("Error when reading line from stdin");
    let result = parse_date_time(&buf);
    if result.is_err() {
        println!("{}", result.unwrap_err());
        println!("Please try again!");
        return ask_for_date_time();
    }
    result.unwrap()
}
