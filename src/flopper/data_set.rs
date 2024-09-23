use miniserde::{Deserialize, Serialize};

use super::params::Params;

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    #[serde(rename = "model_id")]
    pub model: Option<usize>,
    pub params: Params,
}
