use crate::grammar::TermParser;
use log::warn;
use serde::Deserialize;

pub struct Context {
    pub parser: TermParser,
    pub config: Config,
}

#[derive(Deserialize)]
#[serde(default)]
pub struct Config {
    pub src: String,
    pub output: String,
    pub pretty: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            src: "./src".to_string(),
            output: "./out".to_string(),
            pretty: false,
        }
    }
}

const DAISY_CONFIG: &str = "daisy.toml";

pub fn load_config() -> Context {
    let config_str = std::fs::read_to_string(DAISY_CONFIG).unwrap_or_else(|_| {
        warn!("{} not found, using default config", DAISY_CONFIG);
        "".to_string()
    });

    let mut cfg: Config = toml::from_str(&config_str).unwrap();

    let absolute_src = std::fs::canonicalize(&cfg.src).unwrap_or_else(|_| {
        panic!("{} not found, using default src", cfg.src);
    });
    cfg.src = absolute_src.to_str().unwrap().to_string();

    Context {
        parser: TermParser::new(),
        config: cfg,
    }
}
