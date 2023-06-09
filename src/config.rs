use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub cleaning: CleaningConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CleaningConfig {
    pub time: String,
    pub path: String,
}

pub fn read_config_file() -> Result<Config, String> {
    let mut file = File::open("nodemore.yml").map_err(|_| "We cannot read the nodemore.yml! Does it exist?".to_string())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|e| e.to_string())?;

    let config: Config = serde_yaml::from_str(&contents).map_err(|e| e.to_string())?;
    Ok(config)
}

