use miniserde::{Deserialize, Serialize};

use super::status::Status;
#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    pub status: Status,
    pub uuid: String,
}
