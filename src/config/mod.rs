// todo!(Alexander): Add serde to read and serialize Config struct

use std::io::Write;

#[derive(Default)]
pub struct Config {
    pub file_types: Vec<String>,
    pub display: bool,
    pub watch: bool,
}

pub fn read_config() -> Config {
    Config::default()
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
