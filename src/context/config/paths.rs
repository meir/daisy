use std::path::{Path, PathBuf};

use serde::Deserialize;

#[derive(Deserialize, Clone)]
#[serde(default)]
pub struct Paths {
    pub workdir: String,
    pub pages: String,
    pub output: String,
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

impl Paths {
    pub fn get_workdir(&self) -> PathBuf {
        let path = Path::new(&self.workdir);
        path.to_path_buf()
    }

    pub fn get_page_path(&self) -> PathBuf {
        let path = Path::new(&self.workdir).join(&self.pages);
        path.to_path_buf()
    }

    pub fn get_output_path(&self) -> PathBuf {
        let path = Path::new(&self.workdir).join(&self.output);
        path.to_path_buf()
    }
}
