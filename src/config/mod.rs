use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;

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
    println!("{:?}", &conf);
    Ok(conf)
}

impl Write for Config {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        todo!()
    }

    fn flush(&mut self) -> std::io::Result<()> {
        todo!()
    }

    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        todo!()
    }
}

fn add_path(path: &str) {
    todo!()
}

fn remove_path(path: &str) {
    todo!()
}
