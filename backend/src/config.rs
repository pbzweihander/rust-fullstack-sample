use std::path::PathBuf;

use once_cell::sync::Lazy;
use serde::Deserialize;

pub static CONFIG: Lazy<Config> = Lazy::new(|| envy::from_env().unwrap());

fn default_static_file_directory() -> PathBuf {
    PathBuf::from("../frontend/dist")
}

#[derive(Deserialize)]
pub struct Config {
    #[serde(default = "default_static_file_directory")]
    pub static_file_directory: PathBuf,
}
