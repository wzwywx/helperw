use std::{
    default, env,
    ops::{Add, Deref, Sub},
    process::exit,
};

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

fn main() -> Result<(), &'static str> {
    let mut args: Vec<String> = env::args().collect::<Vec<String>>();
    args.remove(0);

    // TODO: remove this once complete
    println!("initial: {:?}", args);

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

    let mut parsed_args = ParsedArguments::default();
    // '-' is reserved by the command to denote the time interval. It can be used
    // to check if the user passed the time interval correctly since '-' expects to have a value before and after it.
    // By checking the existence of the time interval, provided no options are provided, the remaining string
    // is expected to be the description.
    get_time_intervals(&mut args, &mut parsed_args);
    println!("end: {:?}", args);
    Ok(())
}

// TODO: Ponder: Maybe too specific? Maybe abstract-able?
fn get_time_intervals(
    args: &mut Vec<String>,
    parsed_args: &mut ParsedArguments,
) -> Result<(), &'static str> {
    if !args.contains(&"-".to_string()) {
        // TODO: colorize this
        return Err("Time interval is mising. Have you added '-' between your two datetimes e.g. 20000102T0000 - 20000102T0100 ?");
    } else {
        // `unwrap` won't panic here because the existence of dash was checked in the preceding `if` block
        let dash_position = args.iter().position(|x| x == &"-".to_string()).unwrap();
        let before_dash_position = dash_position.checked_sub(1);
        let after_dash_position = dash_position + 1;

        let start_time_exists = before_dash_position.and_then(|idx_before| args.get(idx_before));
        let end_time_exists = args.get(after_dash_position);

        // Make sure an argument exists before and after the dash since a closed interval is assumed i.e. start and end date must always be specified.
        // TODO: A bit repetitive. Minor fix later
        if start_time_exists.is_none() {
            // TODO: Alternative to returning error - return an error and customize the error message based on the returned error
            return Err("Missing a start date. Make sure to put the date before `-`");
        } else if end_time_exists.is_none() {
            return Err("Missing an end date. Make sure to put the date after `-`");
        }

        let before_dash_position = before_dash_position.unwrap();

        // `unwrap` won't panic here because dash positions for before and after was checked in the preceding `if` block
        parsed_args.start_time = Some(args.get(before_dash_position).unwrap().to_string());
        parsed_args.end_time = Some((args.get(after_dash_position).unwrap().to_string()));

        // By removing all the time interval arguments including the dash, it can be deduced that the rest will most likely be the description
        // and the optional tags and projects
        args.remove(after_dash_position);
        args.remove(dash_position);
        args.remove(before_dash_position);
    }
    Ok(())
}

#[derive(Default)]
struct ParsedArguments {
    start_time: Option<String>,
    end_time: Option<String>,
}
