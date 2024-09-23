use miniserde::{Deserialize, Serialize};

use super::{gen_type::GenType, query::Query};
#[derive(Debug, Serialize, Deserialize)]
pub struct Params {
    /// Type of deal
    r#type: GenType,
    /// Number of images
    /// **note**: Now available only one
    #[serde(rename = "numImages")]
    n: usize,
    /// Image size - width
    #[serde(rename = "width")]
    w: usize,
    /// Image size - height
    #[serde(rename = "height")]
    h: usize,
    /// Search query
    #[serde(rename = "generateParams")]
    q: Query,
}
impl Params {
    pub fn new(t: GenType, n: usize, w: usize, h: usize, q: String) -> Self {
        if n != 1 {
            panic!("Now available only one image");
        }
        Self {
            r#type: t,
            // **note**: Now available only one
            n,
            w,
            h,
            q: Query { q },
        }
    }

    pub fn n(&mut self, n: usize) {
        self.n = n;
    }

    pub fn w(&mut self, w: usize) {
        self.w = w;
    }

    pub fn h(&mut self, h: usize) {
        self.h = h;
    }

    pub fn q(&mut self, q: String) {
        self.q = Query { q };
    }
}

impl Default for Params {
    fn default() -> Self {
        Self::new(GenType::Generate, 1, 512, 512, String::new())
    }
}
