use miniserde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Status {
    #[serde(rename = "INITIAL")]
    Initial,
    #[serde(rename = "PROCESSING")]
    Processing,
    #[serde(rename = "FAIL")]
    Fail,
    #[serde(rename = "DONE")]
    Done,
}
