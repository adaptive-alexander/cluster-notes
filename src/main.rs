use std::env;
use std::str::FromStr;

mod actions;
mod config;
mod parser;
mod render;
mod watcher;

fn read_input() -> String {
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Invalid input");
    buffer.trim().to_string()
}

fn parse_args(args: Vec<String>) -> config::Config {
    // Construct options struct

    let mut conf = config::read_config();

    // Check if flag in args
    if args.contains(&"--watch".to_string()) {
        conf.watch = true;
    }
    // Write config flags to config file
    // Could be cleaner if display option is in Config instead of MainOptions
    if args.contains(&"--display".to_string()) {
        todo!("Implement config write")
    }
    if args.contains(&"--no-display".to_string()) {
        todo!("Implement config write")
    }
    conf
}

fn run_cli_mode() {
    // Read configuration
    let conf = config::read_config();

    loop {
        // parse files in watched dirs
        // parser::parse(&conf.file_types);  // Uncomment when implemented

        // Get user input
        let acts = actions::Action::from_str(read_input().as_str());

        // Match Result to run action on Ok
        match acts {
            Ok(a) => actions::actions(&a, &conf),
            Err(e) => eprintln!("Could not perform action due to: {}", e),
        }
    }
}

fn run(conf: config::Config) {
    // Either run in watch or CLI mode
    match conf.watch {
        // Running in watch mode - requires configs to be set to liking beforehand
        true => loop {
            println!("Running in unimplemented watch mode");
            println!("Exiting application.");
            std::process::exit(1);
        },
        // Running in CLI-mode
        false => run_cli_mode(),
    }
}

fn main() {
    run(parse_args(
        env::args().skip(1).collect(), // Input arguments
    ))
}
