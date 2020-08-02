//! Unicode case folding comparisons for UTF-8 `&str` slices.

mod ascii;
mod full;
mod mapping;
mod turkic;

pub use ascii::casecmp as ascii_casecmp;
pub use full::casecmp as unicode_full_casecmp;
pub use turkic::casecmp as unicode_full_turkic_casecmp;
