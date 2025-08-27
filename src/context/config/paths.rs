use serde::Deserialize;

#[derive(Deserialize)]
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
