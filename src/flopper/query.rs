/// Search query pipe
use miniserde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Query {
    /// Search query
    #[serde(rename = "query")]
    pub q: String,
}
