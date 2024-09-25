use miniserde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
/// Type of work - generation
/// Stable: Generate
/// Unimplemented: StyleTransfer, Mixing, Inpainting
pub enum GenType {
    #[serde(rename = "GENERATE")]
    /// Generate image by prompt
    Generate,
    #[serde(rename = "STYLE_TRANSFER")]
    /// Unimplemented
    StyleTransfer,
    #[serde(rename = "MIXING")]
    /// Unimplemented
    Mixing,
    #[serde(rename = "INPAINTING")]
    /// Unimplemented
    Inpainting,
    #[serde(rename = "STICKER")]
    /// Unimplemented
    Sticker,
}
