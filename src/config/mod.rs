use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(default)]
    pub file_types: Vec<String>,
    #[serde(default)]
    pub display: bool,
    #[serde(default)]
    pub watch: bool,
}

pub fn read_config() -> Result<Config, serde_yaml::Error> {
    // Read configuration file
    let conf: Config = serde_yaml::from_reader(fs::File::open("config.yml").unwrap())?;
    Ok(conf)
}

pub fn write_config(conf: &Config) -> Result<(), serde_yaml::Error> {
    serde_yaml::to_writer(
        fs::OpenOptions::new()
            .write(true)
            .open("config.yml")
            .unwrap(),
        conf,
    )?;
    Ok(())
}

pub fn parse_config(args: Vec<String>) -> Config {
    // Construct Config from config.yml
    let mut conf = read_config().unwrap();

    // Check if flag in args
    if args.contains(&"--watch".to_string()) {
        conf.watch = true;
        println!("Set watch to true");
    }
    if args.contains(&"--no-watch".to_string()) {
        conf.watch = false;
        println!("Set watch to false");
    }
    if args.contains(&"--display".to_string()) {
        conf.display = true;
        println!("Set display to true");
    }
    conf
}

fn add_path(path: &str) {
    todo!()
}

fn remove_path(path: &str) {
    todo!()
}
