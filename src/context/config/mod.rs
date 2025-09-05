use log::warn;
use serde::Deserialize;
use std::fs;

mod assets;
mod paths;

use assets::Assets;
use paths::Paths;

#[derive(Deserialize, Clone)]
#[serde(default)]
pub struct Config {
    pub paths: Paths,
    pub assets: Assets,
    pub pretty: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            paths: Paths::default(),
            assets: Assets::default(),
            pretty: false,
        }
    }
}

const DAISY_CONFIG: &str = "daisy.toml";

impl Config {
    pub fn new() -> Self {
        let config_raw = fs::read_to_string(DAISY_CONFIG).unwrap_or_else(|_| {
            warn!("{} not found, using default config", DAISY_CONFIG);
            String::new()
        });

        toml::from_str::<Config>(&config_raw).unwrap().sanitize()
    }

    pub fn sanitize(self) -> Self {
        // todo: sanitize paths
        self
    }
}
