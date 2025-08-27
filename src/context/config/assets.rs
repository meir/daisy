use serde::Deserialize;

#[derive(Deserialize)]
#[serde(default)]
pub struct Assets {
    pub folder: String,
}

impl Default for Assets {
    fn default() -> Self {
        Assets {
            folder: "assets".to_string(),
        }
    }
}
