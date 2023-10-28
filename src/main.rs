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
fn main() -> Result<(), &'static str> {
    let mut args: Vec<String> = env::args().collect::<Vec<String>>();

    // The program name is not necessary for this script
    args.remove(0);

    // TODO: remove this once complete
    println!("initial: {:?}", args);

    let mut parsed_args = ParsedArguments::default();

    //TIME INTERVAL
    //---
    // '-' is reserved by the command to denote the time interval. It can be used
    // to check if the user passed the time interval correctly since '-' expects to have a value before and after it.
    // By checking the existence of the time interval, provided no options are provided, the remaining string
    // is expected to be the description.
    if let Err(msg) = extract_time_intervals(&mut args, &mut parsed_args) {
        return Err(msg);
    }

    //PROJECTS AND TAGS
    //---
    // Projects and tags are commands identified by their `--`. They are optional but they should still be extracted first
    // because everything after their command will be used as their data e.g. `--project project-phoenix` or `--tags project agile code`
    let _ = extract_arguments_and_store(&mut args, &mut parsed_args, "--project");
    let _ = extract_arguments_and_store(&mut args, &mut parsed_args, "--tags");

    //DESCRIPTION
    //---
    // Get the description from the arguments left in the argument array. This assumes that
    // every possible non-description arguments has been parsed and handled
    //
    // Not a big fan of cloning the value inside the Option here, but the presumption is the description
    // is short and shouldn't take too much space. I might opt for a cleaner design later; perhaps use references instead
    // since there's no plan to mutate or own anything.
    parsed_args.description = args.get(0).cloned();

    // TODO: remove debug statement
    println!("end of args: {:?}", args);
    println!("final parsed_args: {:#?}", parsed_args);
    Ok(())
}

// TODO: Ponder: Maybe too specific? Maybe abstract-able? Abstract enough to cover prefix and infix arguments
// Refer to ParsedArguments `start_time` and `end_time` documentation for the general idea on what is expected
// of a time interval. And as of now, even though timewarrior does expect certain format for time, it's not
// the job of the script and of course, this function, to check whether the passed interval is correct or not.
//
// That capability might be added in the future.
fn extract_time_intervals(
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

        // By removing all the time interval arguments including the dash, it can be deduced that the remaining arguments will most likely be the description
        // and the optional tags and projects
        args.remove(after_dash_position);
        args.remove(dash_position);
        args.remove(before_dash_position);
    }
    Ok(())
}

// A general function that can parse and extract specific arguments and optionally, the input/data after it. For example,
// `--project sleep` and `-v` can both be extracted and then stored in a struct. For now that struct is `ParsedArguments`.
//
// Another thing worth mentioning, this function mutates the argument array. I'll add a non-mutating version of this function later.
//
// TODO: I don't think mutating ParsedArguments and args is necessary. They should just return the info and let the caller do what they want.
// Unless, I want to pull the complexity downward especially if the behaviour is common enough to handle by the function itself.
// And I think this function is dependent on parsed_args and they share information that doesn't seem to be necessary.
//
// TODO: I might need to make a more general function for this. This function is too specific to the script's need.
fn extract_arguments_and_store(
    args: &mut Vec<String>,
    parsed_args: &mut ParsedArguments,
    command: &str,
) -> Result<(), &'static str> {
    // TODO: this should be in a loop with an array of approved prefixes
    if !(command == "--project" || command == "--tags") {
        return Err("Prefix can only be --project or --tag");
    }

    let command_position: usize = if let Some((idx, _)) = args
        .iter()
        .enumerate()
        .find(|(_, arg)| arg.contains(command))
    {
        idx
    } else {
        return Err("Command not found");
    };

    let input_error = Err("Missing input to --project");

    match command {
        // TODO: hackyish. because this function is (supposed to be) quite general, I don't know whether there should be a check
        // that determines whether the input that follows the command is an input to the command or another command.
        // It's hard to know because I haven't decided on a proper commands/options format
        "--project" => {
            let project_name = args.get(command_position + 1).cloned();

            match project_name {
                Some(_) if project_name.as_ref().unwrap() == &"--tags".to_string() => {
                    return input_error;
                }
                Some(_) => parsed_args.project = project_name,
                None => {
                    return input_error;
                }
            }
        }
        "--tags" => {
            let mut tags = Vec::<String>::new();

            //TODO: Is this possible to be mapped? Seems weird because there's a break in the control flow. Later.
            for arg in &args[command_position + 1..] {
                if arg == "--project" {
                    break;
                }
                tags.push(arg.to_string());
            }

            if tags.is_empty() {
                return input_error;
            }
            // let project_name = args.get(command_position + 1).cloned();
            parsed_args.tags = Some(tags);
        }
        _ => return input_error,
    }
    Ok(())
}

#[derive(Default, Debug)]
struct ParsedArguments {
    // `timew track` requires a relatively flexible time interval as documented [here](https://timewarrior.net/docs/interval/)
    // and [here](https://timewarrior.net/docs/dates/).
    // But, supporting the whole syntax in `helperw` might be difficult, so for now,
    // the expected datetime syntax is strictly YYYYMMDDThhmm.
    start_time: Option<String>,
    end_time: Option<String>,

    // Since both `task` and `timew` require a description, a lone string without
    // any option before it will be used as the description
    description: Option<String>,

    project: Option<String>,
    tags: Option<Vec<String>>,
}

#[cfg(debug_assertions)]
fn execute_warriors() {}
