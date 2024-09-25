pub mod error;
pub mod flopper;
pub mod macros;
pub mod result;

pub mod prelude {
    pub use crate::error::Errors;
    pub use crate::flop;
    pub use crate::flopper::{gen_type::GenType, params::Params, Flopper};
    pub use crate::result::Result;
}
