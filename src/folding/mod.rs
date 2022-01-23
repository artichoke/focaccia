//! Unicode case folding comparisons for UTF-8 `&str` slices.

mod ascii;
mod full;
mod mapping;
mod turkic;

pub use ascii::casecmp as ascii_casecmp;
pub use ascii::case_eq as ascii_case_eq;
pub use full::casecmp as unicode_full_casecmp;
pub use full::case_eq as unicode_full_case_eq;
pub use turkic::casecmp as unicode_full_turkic_casecmp;
pub use turkic::case_eq as unicode_full_turkic_case_eq;
