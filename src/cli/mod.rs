pub mod cron;
pub mod init;
pub mod list;
pub mod new;
pub mod remove;

use crate::{db, utils, PROGRAM_CRON_DESC, PROGRAM_REMOVE_DESC, TWITIMER_VER};
use crate::{PROGRAM_INIT_DESC, PROGRAM_LIST_DESC, PROGRAM_NAME, PROGRAM_NEW_DESC};
use args;
use getopts::Occur;

pub fn print_help() {
    println!(
        "
    Twitimer {} by Richard Chen 2022 (DB: {})\n
    Usage: 
    \tShow this page:
    \t\t$ twitimer help
    \tInit or update twitimer configuration:
    \t\t$ twitimer init --help
    \tCreate a new twitimer task:
    \t\t$ twitimer new --help
    \tList your twitimer task(s)
    \t\t$ twitimer list --help
    \tRemove your twitimer task(s)
    \t\t$ twitimer remove --help
    \tCheck for actions
    \t\t$ twitimer cron --help\n",
        TWITIMER_VER,
        db::db_path()
    )
}

pub fn init_args() -> args::Args {
    let mut args = args::Args::new(
        utils::str::str_join(PROGRAM_NAME, "init").as_str(),
        PROGRAM_INIT_DESC,
    );
    args.flag("h", "help", "Print the help page");
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
    // args.flag("", "no-msg", "Do not list message column");
    args.flag("j", "json", "Print or save as json");
    args.option(
        "i",
        "id",
        "Your task id or id range",
        "12 or 1,3,7 or 1-88 or 1,3,5,7-11",
        Occur::Optional,
        None,
    );
    // args.option(
    //     "m",
    //     "max",
    //     "The maximum tasks that will be listed (decreased id)",
    //     "MAX",
    //     Occur::Optional,
    //     None,
    // );
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

pub fn remove_args() -> args::Args {
    let mut args = args::Args::new(
        utils::str::str_join(PROGRAM_NAME, "remove").as_str(),
        PROGRAM_REMOVE_DESC,
    );
    args.flag("h", "help", "Print the help page");
    args.flag("", "ignore", "Ignore warnings and handle as default");
    args.option(
        "i",
        "id",
        "Your task id(s)",
        "12 or 1,3,7 or 1-88 or 1,3,5,7-11",
        Occur::Optional,
        None,
    );
    args
}

pub fn cron_args() -> args::Args {
    let mut args = args::Args::new(
        utils::str::str_join(PROGRAM_NAME, "cron").as_str(),
        PROGRAM_CRON_DESC,
    );
    args.flag("h", "help", "Print the help page");
    args.flag("", "remove", "Remove after end done");
    args
}
