pub mod init;
pub mod new;

use crate::{utils, PROGRAM_CRON_DESC, TWITIMER_VER};
use crate::{
    PROGRAM_EDIT_DESC, PROGRAM_INIT_DESC, PROGRAM_LIST_DESC, PROGRAM_NAME, PROGRAM_NEW_DESC,
};
use args;
use getopts::Occur;
use std::io;
use std::io::{BufRead, Write};

pub fn print_help() {
    println!(
        "
    Twitimer {} by Richard Chen (i@iochen.com) 2022\n
    Usage: 
    \tShow this page:
    \t\t$ twitimer help
    \tInit or update twitimer configuration:
    \t\t$ twitimer init --help
    \tCreate a new twitimer task:
    \t\t$ twitimer new --help
    \tList your twitimer task(s)
    \t\t$ twitimer list --help
    \tEdit your twitimer task
    \t\t$ twitimer edit --help
    \tCheck for actions
    \t\t$ twitimer cron --help\n",
        TWITIMER_VER
    )
}

pub fn init_args() -> args::Args {
    let mut args = args::Args::new(
        utils::str::str_join(PROGRAM_NAME, "init").as_str(),
        PROGRAM_INIT_DESC,
    );
    args.flag("h", "help", "Print the help page");
    args.flag("u", "update", "Update credentials");
    args.flag("s", "silent", "Select all y/n choices as default");
    args.option(
        "",
        "consumer-key",
        "Twitter developer consumer key",
        "CON_KEY",
        Occur::Optional,
        None,
    );
    args.option(
        "",
        "consumer-secret",
        "Twitter developer consumer secret",
        "CON_SEC",
        Occur::Optional,
        None,
    );
    args.option(
        "",
        "access-key",
        "Twitter user access key",
        "ACC_KEY",
        Occur::Optional,
        None,
    );
    args.option(
        "",
        "access-secret",
        "Twitter user access secret",
        "ACC_SEC",
        Occur::Optional,
        None,
    );
    args
}

pub fn new_args() -> args::Args {
    let mut args = args::Args::new(
        utils::str::str_join(PROGRAM_NAME, "new").as_str(),
        PROGRAM_NEW_DESC,
    );
    args.flag("h", "help", "Print the help page");
    args.flag("s", "silent", "Select all y/n choices as default");
    args.option(
        "m",
        "message",
        "Your message content",
        "\"Hello, world!\\n你好\\nこんにちは 👋\"",
        Occur::Optional,
        None,
    );
    args.option(
        "s",
        "start",
        "The start time of your tweet",
        "\"2022-02-12 13:23:45 +8\" or UNIX_TIMESTAMP",
        Occur::Optional,
        None,
    );
    args.option(
        "e",
        "end",
        "\"2022-02-12 13:23:45 +0\" or UNIX_TIMESTAMP or \"None\"",
        "END",
        Occur::Optional,
        None,
    );
    // args.option(
    //     "n",
    //     "note",
    //     "The note to be attached to the task",
    //     "NOTE",
    //     Occur::Optional,
    //     None,
    // );
    args.option(
        "",
        "import",
        "Import from json file (More in README.md)",
        "filename.json",
        Occur::Optional,
        None,
    );
    args
}

pub fn list_args() -> args::Args {
    let mut args = args::Args::new(
        utils::str::str_join(PROGRAM_NAME, "list").as_str(),
        PROGRAM_LIST_DESC,
    );
    args.flag("h", "help", "Print the help page");
    args.flag("s", "silent", "Select all y/n choices as default");
    args.flag("", "no-msg", "Do not list message column");
    args.flag("j", "json", "Print or save as json");
    args.option(
        "i",
        "id",
        "Your task id or id range",
        "12 or 1,3,7 or 1-88 or 1,3,5,7-11",
        Occur::Optional,
        None,
    );
    args.option(
        "m",
        "max",
        "The maximum tasks that will be listed",
        "MAX",
        Occur::Optional,
        None,
    );
    args.option(
        "o",
        "output",
        "Save to file as json",
        "filename.json",
        Occur::Optional,
        None,
    );
    // args.option(
    //     "n",
    //     "note",
    //     "Filter by the note attached to the task",
    //     "NOTE",
    //     Occur::Optional,
    //     None,
    // );
    args
}

pub fn edit_args() -> args::Args {
    let mut args = args::Args::new(
        utils::str::str_join(PROGRAM_NAME, "edit").as_str(),
        PROGRAM_EDIT_DESC,
    );
    args.flag("h", "help", "Print the help page");
    args.flag("s", "silent", "Select all y/n choices as default");
    args.flag("", "ignore", "Ignore warnings and handle as default");
    args.flag("r", "rm", "Remove task from twitimer");
    args.option(
        "i",
        "id",
        "Your task id (single)",
        "ID",
        Occur::Optional,
        None,
    );
    args.option(
        "m",
        "message",
        "The message that you want to changed to",
        "\"Have a nice sleep!\\n晚安\\nおやすみなさい 😴\"",
        Occur::Optional,
        None,
    );
    args.option(
        "s",
        "start",
        "The start time that you want to changed to",
        "\"2022-02-12 13:23:45 +9\" or UNIX_TIMESTAMP",
        Occur::Optional,
        None,
    );
    args.option(
        "e",
        "end",
        "The end time that you want to changed to",
        "\"2022-02-12 13:23:45 -1\" or UNIX_TIMESTAMP",
        Occur::Optional,
        None,
    );
    // args.option(
    //     "n",
    //     "note",
    //     "The note that you want to changed to",
    //     "NOTE",
    //     Occur::Optional,
    //     None,
    // );
    args
}

pub fn cron_args() -> args::Args {
    let mut args = args::Args::new(
        utils::str::str_join(PROGRAM_NAME, "cron").as_str(),
        PROGRAM_CRON_DESC,
    );
    args.flag("h", "help", "Print the help page");
    args
}
