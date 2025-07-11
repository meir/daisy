use std::{
    fs,
    io::Error,
    path::{Path, PathBuf},
    rc::Rc,
};

use crate::{grammar::DaisyParser, resolver::Resource};
use log::warn;
use serde::Deserialize;

pub struct Context {
    pub parser: DaisyParser,
    pub resources: Vec<Rc<Resource>>,
    pub config: Config,
}

#[derive(Deserialize)]
#[serde(default)]
pub struct Config {
    pub paths: Paths,
    pub assets: Assets,
    pub pretty: bool,
}

#[derive(Deserialize)]
#[serde(default)]
pub struct Paths {
    pub workdir: String,
    pub pages: String,
    pub output: String,
}

#[derive(Deserialize)]
#[serde(default)]
pub struct Assets {
    pub folder: String,
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

impl Default for Paths {
    fn default() -> Self {
        Paths {
            workdir: ".".to_string(),
            pages: "src".to_string(),
            output: "site".to_string(),
        }
    }
}

impl Default for Assets {
    fn default() -> Self {
        Assets {
            folder: "assets".to_string(),
        }
    }
}

const DAISY_CONFIG: &str = "daisy.toml";

impl Context {
    pub fn load_config() -> Self {
        let config_str = std::fs::read_to_string(DAISY_CONFIG).unwrap_or_else(|_| {
            warn!("{} not found, using default config", DAISY_CONFIG);
            "".to_string()
        });

        let mut cfg: Config = toml::from_str(&config_str).unwrap();

        let absolute_src = std::fs::canonicalize(&cfg.paths.workdir).unwrap_or_else(|_| {
            panic!("{} not found, using default src", cfg.paths.workdir);
        });
        cfg.paths.workdir = absolute_src.to_str().unwrap().to_string();

        Context {
            parser: DaisyParser::new(),
            resources: vec![],
            config: cfg,
        }
    }

    pub fn get_output_path(&self, src: &str) -> Result<PathBuf, Error> {
        let mut path = Path::new(src);
        let name = path.file_stem().unwrap();
        let pathbuf = path.parent().unwrap().join(name);
        path = pathbuf.as_path();

        if path.extension().is_some() {
            std::path::absolute(&format!(
                "{}/{}/{}",
                self.config.paths.workdir,
                self.config.paths.output,
                path.to_str().unwrap(),
            ))
        } else {
            if name == "index" {
                path = path.parent().unwrap();
            }

            std::path::absolute(&format!(
                "{}/{}/{}/index.html",
                self.config.paths.workdir,
                self.config.paths.output,
                path.to_str().unwrap(),
            ))
        }
    }

    pub fn save_content(&self, path: &str, content: &str) -> String {
        let output_path = Path::new(path);

        fs::create_dir_all(output_path.parent().unwrap()).unwrap_or_else(|err| {
            panic!("Failed to create directory: {}: {}", path, err);
        });

        fs::write(&output_path, content).unwrap_or_else(|err| {
            panic!("Failed to write file: {}: {}", path, err);
        });

        output_path.to_str().unwrap().to_string()
    }
}
