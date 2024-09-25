use miniserde::{Deserialize, Serialize};

use super::status::Status;
#[derive(Debug, Serialize, Deserialize)]
pub struct Work {
    pub uuid: String,
    pub status: Status,
    pub images: Option<Vec<String>>,
    #[serde(rename = "errorDescription")]
    pub error: Option<String>,
    pub censored: bool,
    #[serde(rename = "generationTime")]
    pub generation_time: Option<usize>,
}
