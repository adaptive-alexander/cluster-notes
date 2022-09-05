use std::str::FromStr;
use std::{env, thread, time};

mod actions;
mod config;
mod parser;
mod render;
mod watcher;

#[derive(Default)]
struct MainOptions {
    watch: bool,
    display: bool,
}

fn read_input() -> String {
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Invalid input");
    buffer.trim().to_string()
}

fn parse_args(args: Vec<String>) -> MainOptions {
    // Construct options struct
    let mut main_opts = MainOptions::default();

    let conf = config::read_config();

    // Check if flag in args
    if args.contains(&"--watch".to_string()) {
        main_opts.watch = true;
    }
    // Write config flags to config file
    if args.contains(&"--display".to_string()) {
        main_opts.display = true;
    }
    if args.contains(&"--nodisplay".to_string()) {
        main_opts.display = false;
    }
    main_opts
}

fn run(options: MainOptions) {
    // Read configuration
    // let conf = config::read_config();  // Uncomment when implemented
    match options.watch {
        // Running in watch mode - requires configs to be set to liking beforehand
        true => loop {
            println!("Running in unimplemented watch mode");
            thread::sleep(time::Duration::from_millis(1000))
        },
        // Running in CLI-mode
        false => loop {
            // parser::parse(&conf.file_types);
            let acts = actions::Actions::from_str(read_input().as_str());
            match acts {
                Ok(a) => {
                    actions::actions(&a);
                    if a == actions::Actions::Render {
                        match options.display {
                            true => actions::actions(&actions::Actions::Display),
                            false => {}
                        }
                    }
                }
                Err(e) => eprintln!("Could not perform action due to: {}", e),
            }
        },
    }
}

fn main() {
    run(parse_args(env::args().skip(1).collect())) // Input arguments))
}
