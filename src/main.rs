use crate::actions::{actions, Actions};
use std::str::FromStr;
use std::{env, thread, time};

mod actions;
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

    // Check if flag in args
    if args.contains(&"--watch".to_string()) {
        main_opts.watch = true;
    }
    if args.contains(&"--display".to_string()) {
        main_opts.display = true;
    }
    if args.contains(&"--nodisplay".to_string()) {
        main_opts.display = false;
    }
    main_opts
}

fn run(options: MainOptions) {
    match options.watch {
        true => loop {
            println!("Running in unimplemented watch mode");
            thread::sleep(time::Duration::from_millis(1000))
        },
        false => loop {
            let acts = Actions::from_str(read_input().as_str());
            match acts {
                Ok(a) => {
                    actions(&a);
                    if a == Actions::Render {
                        match options.display {
                            true => actions(Actions::Display),
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
    run(parse_args(env::args().skip(1).collect()));
}
