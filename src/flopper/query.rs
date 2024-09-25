use miniserde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
/// Search query
pub struct Query {
    /// Search query
    #[serde(rename = "query")]
    pub q: String,
}
