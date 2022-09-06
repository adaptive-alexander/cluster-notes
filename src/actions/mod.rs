use super::read_input;
use crate::config::{parse_config, read_config, Config};
use crate::watcher;
use crate::{config, render};
use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;

#[derive(Debug)]
pub struct ActionError {
    message: String,
}

impl fmt::Display for ActionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} not being a valid action.", self.message)
    }
}

#[derive(PartialEq, Eq)]
pub enum Action {
    Config,
    Render,
    Display,
}

impl FromStr for Action {
    type Err = ActionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "config" => Ok(Action::Config),
            "render" => Ok(Action::Render),
            "display" => Ok(Action::Display),
            _ => Err(ActionError {
                message: format!("\"{}\"", s),
            }),
        }
    }
}

fn setup_configs() {
    println!("Implement config setup!"); // todo!(Alexander)
    let mut args: Vec<String> = Vec::new();
    println!("Enter config to change or \"done\"");
    loop {
        let inp = read_input();
        match inp.as_str() {
            "done" => break,
            _ => args.push(inp),
        }
    }
    let conf = parse_config(args);
    config::write_config(&conf).expect("Failed to write config");
}

pub fn actions(acts: &Action, conf: &Config) {
    match acts {
        Action::Config => setup_configs(),
        Action::Render => {
            render::render();
            // Also display if option set to true
            match conf.display {
                true => watcher::display(),
                false => {}
            }
        }
        Action::Display => watcher::display(),
    }
}
