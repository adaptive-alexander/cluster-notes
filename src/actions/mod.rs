use crate::render;
use crate::watcher;
use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;

#[derive(Debug)]
pub struct ActionError {
    message: String,
}

impl fmt::Display for ActionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} is not a valid action.", self.message)
    }
}

#[derive(PartialEq, Eq)]
pub enum Actions {
    Config,
    Render,
    Display,
}

impl FromStr for Actions {
    type Err = ActionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "config" => Ok(Actions::Config),
            "render" => Ok(Actions::Render),
            _ => Err(ActionError {
                message: format!("\"{}\"", s),
            }),
        }
    }
}

fn setup_configs() {
    println!("Implement config setup!") // todo!(Alexander)
}

pub fn actions(acts: &Actions) {
    match acts {
        Actions::Config => setup_configs(),
        Actions::Render => render::render(),
        Actions::Display => watcher::display(),
    }
}
