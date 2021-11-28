use structopt::StructOpt;

/// Start or stop a pomodoro instance.
#[derive(StructOpt)]
struct Cli {
    /// start or stop
    command: String,
    /// work or break
    unit: String,
}

fn main() {
    let args = Cli::from_args();

    let command_message = match args.command.as_str() {
        "start" => "starting timer",
        "stop" => "stopping timer",
        _ => "try running pom again with 'start' or 'stop' as the first arguments"
    };

    let unit_message = match args.unit.as_str() {
        "work" => "work",
        "break" => "break",
        _ => "second argument must be either 'work' or 'break'"
    };

    println!("{}\n{}", command_message, unit_message);
}
