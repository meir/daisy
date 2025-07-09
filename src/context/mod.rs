use std::{
    fs,
    io::Error,
    path::{Path, PathBuf},
};

use crate::{ast::environment::Scope, grammar::DaisyParser, resolver::file::File};
use log::warn;
use serde::Deserialize;

pub struct Context {
    pub parser: DaisyParser,
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
            config: cfg,
        }
    }

    pub fn get_page_output_path(&self, src: &str) -> Result<PathBuf, Error> {
        let mut path = Path::new(src);
        let name = path.file_stem().unwrap();
        let pathbuf = path.parent().unwrap().join(name);
        path = pathbuf.as_path();
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

    pub fn save_page(&self, path: &str, content: &str) -> String {
        let src = Path::new(path).strip_prefix(format!(
            "{}/{}",
            self.config.paths.workdir, self.config.paths.pages
        ));
        let output_path = self
            .get_page_output_path(src.unwrap().to_str().unwrap())
            .unwrap();

        fs::create_dir_all(output_path.parent().unwrap()).unwrap_or_else(|err| {
            panic!("Failed to create directory: {}: {}", path, err);
        });

        fs::write(&output_path, content).unwrap_or_else(|err| {
            panic!("Failed to write file: {}: {}", path, err);
        });

        output_path.to_str().unwrap().to_string()
    }

    pub fn asset_folder(&self) -> PathBuf {
        let asset_folder = format!(
            "{}/{}/{}",
            self.config.paths.workdir, self.config.paths.output, self.config.assets.folder
        );
        let asset_folder = Path::new(asset_folder.as_str());

        if !asset_folder.exists() {
            fs::create_dir_all(asset_folder).unwrap_or_else(|err| {
                panic!(
                    "Failed to create asset directory: {}: {}",
                    asset_folder.display(),
                    err
                );
            });
        }

        asset_folder.to_path_buf()
    }

    pub fn save_asset(&self, name: &str, content: &str) -> String {
        let asset_folder = self.asset_folder();

        let uuid = uuid::Uuid::new_v4();
        let mut asset_path = asset_folder.join(format!("{}-{}", uuid, name));
        fs::write(&asset_path, content).unwrap_or_else(|err| {
            panic!(
                "Failed to write asset file: {}: {}",
                asset_path.display(),
                err
            );
        });

        let prefix = format!("{}/{}", self.config.paths.workdir, self.config.paths.output);
        asset_path = asset_path.strip_prefix(prefix).unwrap().to_path_buf();
        asset_path = PathBuf::from(format!("/{}", asset_path.to_str().unwrap()));

        asset_path.to_str().unwrap().to_string()
    }

    pub fn use_asset(&self, path: &str, literal: bool) -> String {
        let asset_folder = self.asset_folder();

        let mut asset_path = if literal {
            let name = Path::new(path)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_else(|| {
                    panic!("Failed to get file name from path: {}", path);
                });
            asset_folder.join(name)
        } else {
            let uuid = uuid::Uuid::new_v4();
            let name = Path::new(path)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_else(|| {
                    panic!("Failed to get file name from path: {}", path);
                });
            asset_folder.join(format!("{}-{}", uuid, name))
        };

        fs::copy(path, &asset_path).unwrap_or_else(|err| {
            panic!("Failed to copy asset file: {}: {}", path, err);
        });

        let prefix = format!("{}/{}", self.config.paths.workdir, self.config.paths.output);
        asset_path = asset_path.strip_prefix(prefix).unwrap().to_path_buf();
        asset_path = PathBuf::from(format!("/{}", asset_path.to_str().unwrap()));

        asset_path.to_str().unwrap().to_string()
    }
}
