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
    println!("{:?}", args);

    // Since both `task` and `timew` requires a description, a lone string without
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
}
