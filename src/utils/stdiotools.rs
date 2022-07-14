use std::io::{self, BufRead, Write};

pub fn yes_or_no(prompt: &str, default: Option<bool>, silent: bool) -> bool {
    if silent && default.is_some() {
        return default.unwrap();
    }

    if default.is_none() {
        print!("{} (y/n): ", prompt);
    } else {
        if default.unwrap() == true {
            print!("{} (Y/n): ", prompt);
        } else {
            print!("{} (y/N): ", prompt);
        }
    }

    io::stdout().flush().expect("Error when flushing stdout");

    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("Error when reading stdin");

    let buf = buf.trim().to_lowercase();

    if buf.is_empty() && default.is_some() {
        return default.unwrap();
    }

    if buf.eq(&String::from("y")) || buf.eq(&String::from("yes")) {
        return true;
    }

    if buf.eq(&String::from("n")) || buf.eq(&String::from("no")) {
        return false;
    }

    yes_or_no(prompt, default, silent)
}

pub fn ask_for_set(subject: &str) -> String {
    print!("Set {} to: ", subject);
    io::stdout().flush().expect("Error when flushing stdout");
    let mut buf = String::new();
    io::stdin()
        .lock()
        .read_line(&mut buf)
        .expect("Error when reading line from stdin");
    buf = buf.trim().to_string();
    if buf.is_empty() {
        return ask_for_set(subject);
    }
    buf
}

pub fn ask_correct(subject: &str, object: Option<String>, silent: bool) -> String {
    if object.is_none() {
        return ask_for_set(subject);
    }

    let object = object.unwrap();

    if object.is_empty() {
        return ask_for_set(subject);
    }

    println!("{} is set to {},", subject, object);
    let is_correct = yes_or_no("Is it correct?", Some(true), silent);
    if is_correct {
        return object.trim().to_string();
    }

    return ask_for_set(subject);
}

pub fn clear_terminal() {
    // reference: https://stackoverflow.com/questions/34837011/how-to-clear-the-terminal-screen-in-rust-after-a-new-line-is-printed
    // 😓
    print!("\x1B[2J\x1B[1;1H");
}

// fn get_string(prompt: &str) -> Result<String, io::Error> {
//     print!("{}", prompt);
//     io::stdout().flush()?;
//     let mut buf = String::new();
//     io::stdin().lock().read_line(&mut buf)?;
//     let buf = buf.trim().to_string();
//     Ok(buf)
// }
