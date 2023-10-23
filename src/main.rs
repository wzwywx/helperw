use std::env;

/// The command takes in a series of tags and descriptions, plus the task's time interval
/// and then log the data in both taskwarrior and timewarrior
///
/// The main use case for this is to track both task and the time it took to complete it after the
/// task completes without invoking two separate commands `task` and `timew`.
///
/// Usage: helperw DESCRIPTION START_TIME - END_TIME [OPTIONS...]
///
/// Options:
/// -p <project name>
/// -t <tags...>

fn main() {
    let mut args: Vec<String> = env::args().collect::<Vec<String>>();
    args.remove(0);

    // TODO: remove this once complete
    println!("{:?}", args);

    // Since both `task` and `timew` require a description, a lone string without
    // any options will be used as the description
    let description: String;

    // `timew track` requires a time interval which is quite flexible as documented [here](https://timewarrior.net/docs/interval/)
    // and [here](https://timewarrior.net/docs/dates/).
    // But, supporting the whole syntax in `helperw` might be difficult, so for now,
    // the expected datetime syntax is strictly YYYYMMDDThhmm.
    let start_time: String;
    let end_time: String;

    let project: Option<String> = None;
    let tags: Option<Vec<String>> = None;

    // '-' is reserved by the command to denote the time interval. It can be used
    // to check if the user passed the time interval correctly since '-' expects to have a value before and after it.
    // By checking the existence of the time interval, provided no options are provided, the remaining string
    // is expected to be the description.
    if !args.contains(&"-".to_string()) {
        // TODO: colorize this
        eprintln!("Time interval is mising. Have you added '-' between your two datetimes e.g. 20000102T0000 - 20000102T0100 ?");
    }
}
