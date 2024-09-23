use miniserde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Model {
    pub id: usize,
    pub name: String,
    pub version: f64,
    pub r#type: String,
}
