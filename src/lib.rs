#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![warn(missing_docs, intra_doc_link_resolution_failure)]
#![warn(missing_debug_implementations)]
#![warn(missing_copy_implementations)]
#![warn(rust_2018_idioms)]
#![warn(trivial_casts, trivial_numeric_casts)]
#![warn(unused_qualifications)]
#![warn(variant_size_differences)]
#![forbid(unsafe_code)]

//! Unicode case folding methods for case-insensitive string comparisons.
//!
//! Focaccia supports full, ASCII, and Turkic [Unicode case folding] equality
//! comparisons. ASCII folding supports determining case-insensitive
//! [`Ordering`].
//!
//! The primary entrypoint to Focaccia is the [`CaseFold`] enum. Focaccia also
//! provides free functions for each case folding scheme.
//!
//! # Examples
//!
//! For Unicode text, Focaccia recommends the [`Full`](CaseFold::Full) folding
//! scheme. To make an equality comparison:
//!
//! ```
//! # use focaccia::CaseFold;
//! let fold = CaseFold::Full;
//! assert!(fold.casecmp("MASSE", "Maße"));
//! assert!(!fold.casecmp("São Paulo", "Sao Paulo"));
//! ```
//!
//! For text known to be ASCII, Focaccia can make a more performant comparison
//! check:
//!
//! ```
//! # use focaccia::CaseFold;
//! let fold = CaseFold::Ascii;
//! assert!(fold.casecmp("Crate: focaccia", "Crate: FOCACCIA"));
//! assert!(!fold.casecmp("Fabled", "failed"));
//! ```
//!
//! ASCII case comparison can be checked on a byte slice:
//!
//! ```
//! # use core::cmp::Ordering;
//! assert_eq!(
//!     focaccia::ascii_casecmp(b"Artichoke Ruby", b"artichoke ruby"),
//!     Ordering::Equal,
//! );
//! ```
//!
//! Turkic case folding is similar to full case folding with additional mappings
//! for [dotted and dotless I]:
//!
//! ```
//! # use focaccia::CaseFold;
//! let fold = CaseFold::Turkic;
//! assert!(fold.casecmp("İstanbul", "istanbul"));
//! assert!(!fold.casecmp("İstanbul", "Istanbul"));
//! ```
//!
//! # `no_std`
//!
//! Focaccia is `no_std` compatible. By default, Focaccia builds with its
//! **std** feature enabled to implement [`Error`].
//!
//! When built without the **std** feature, Focaccia does not link to `alloc`.
//!
//! # Crate features
//!
//! All features are enabled by default.
//!
//! - **std** - Enable linking to the [Rust Standard Library]. Enabling this
//!   feature adds [`Error`] implementations to error types in this crate.
//!
//! [Unicode case folding]: https://www.w3.org/International/wiki/Case_folding
//! [`Ordering`]: core::cmp::Ordering
//! [dotted and dotless I]: https://en.wikipedia.org/wiki/Dotted_and_dotless_I
//! [Rust Standard Library]: https://doc.rust-lang.org/stable/std/index.html
//! [`Error`]: https://doc.rust-lang.org/stable/std/error/trait.Error.html

#![cfg_attr(not(feature = "std"), no_std)]
#![doc(html_root_url = "https://docs.rs/focaccia/1.0.0")]

// Ensure code blocks in README.md compile
#[cfg(doctest)]
macro_rules! readme {
    ($x:expr) => {
        #[doc = $x]
        mod readme {}
    };
    () => {
        readme!(include_str!("../README.md"));
    };
}
#[cfg(doctest)]
readme!();

use core::convert::TryFrom;
use core::fmt;

mod folding;

pub use folding::{ascii_casecmp, unicode_full_casecmp, unicode_full_turkic_casecmp};

/// Unicode case folding strategies.
///
/// Unicode case folding data supports both implementations that require simple
/// case foldings (where string lengths don't change), and implementations that
/// allow full case folding (where string lengths may grow). Note that where
/// they can be supported, the full case foldings are superior: for example,
/// they allow "MASSE" and "Maße" to match.
///
/// The `CaseFold` enum intends to support the [folding strategies available in
/// Ruby].
///
/// [folding strategies available in Ruby]: https://ruby-doc.org/core-2.6.3/String.html#method-i-downcase
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum CaseFold {
    /// Full Unicode case mapping, suitable for most languages (see [`Turkic`]
    /// and [`Lithuanian`] strategies for exceptions). Context-dependent case
    /// mapping as described in Table 3-14 of the Unicode standard is currently
    /// not supported.
    ///
    /// [`Turkic`]: Self::Turkic
    /// [`Lithuanian`]: Self::Lithuanian
    Full,
    /// Only the ASCII region, i.e. the characters "A" to "Z" and "a" to "z",
    /// are affected.
    Ascii,
    /// Full Unicode case mapping, adapted for Turkic languages (Turkish,
    /// Azerbaijani, ...). This means that upper case I is mapped to lower case
    /// dotless i, and so on.
    Turkic,
    /// Currently, just full Unicode case mapping. In the future, full Unicode
    /// case mapping adapted for Lithuanian (keeping the dot on the lower case i
    /// even if there is an accent on top).
    Lithuanian,
}

impl Default for CaseFold {
    /// Default to full Unicode case folding.
    ///
    /// See [`CaseFold::Full`].
    #[inline]
    fn default() -> Self {
        Self::Full
    }
}

impl CaseFold {
    /// Make a case-insensitive string comparison based on the dispatching
    /// folding strategy.
    ///
    /// Return `true` if the given strings match when folding case. For example,
    /// when using `CaseFold::Full`, `ß` is considered equal to `ss`. The
    /// differences between the folding strategies are documented on the
    /// variants of the [`CaseFold`] enum.
    ///
    /// This function is a wrapper around the underlying scheme-specific
    /// functions.
    #[inline]
    #[must_use]
    pub fn casecmp(self, left: &str, right: &str) -> bool {
        match self {
            Self::Full | Self::Lithuanian => unicode_full_casecmp(left, right),
            Self::Ascii => left.eq_ignore_ascii_case(right),
            Self::Turkic => unicode_full_turkic_casecmp(left, right),
        }
    }
}

/// Error type for returned when a folding scheme could not be resolved in a
/// [`TryFrom`] implementation.
#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NoSuchCaseFoldingScheme(());

impl NoSuchCaseFoldingScheme {
    /// Construct a new [`NoSuchCaseFoldingScheme`] error.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl fmt::Display for NoSuchCaseFoldingScheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("No such Unicode case folding scheme exists")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for NoSuchCaseFoldingScheme {}

impl TryFrom<Option<&str>> for CaseFold {
    type Error = NoSuchCaseFoldingScheme;

    fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
        match value {
            None => Ok(Self::Full),
            Some("ascii") => Ok(Self::Ascii),
            Some("turkic") => Ok(Self::Turkic),
            Some("lithuanian") => Ok(Self::Lithuanian),
            Some(_) => Err(NoSuchCaseFoldingScheme::new()),
        }
    }
}

impl TryFrom<Option<&[u8]>> for CaseFold {
    type Error = NoSuchCaseFoldingScheme;

    fn try_from(value: Option<&[u8]>) -> Result<Self, Self::Error> {
        match value {
            None => Ok(Self::Full),
            Some(scheme) if scheme == b"ascii" => Ok(Self::Ascii),
            Some(scheme) if scheme == b"turkic" => Ok(Self::Turkic),
            Some(scheme) if scheme == b"lithuanian" => Ok(Self::Lithuanian),
            Some(_) => Err(NoSuchCaseFoldingScheme::new()),
        }
    }
}

// Tests using IDN case folding test vectors:
#[cfg(test)]
mod tests {
    use crate::CaseFold;

    // https://tools.ietf.org/html/draft-josefsson-idn-test-vectors-00#section-4.2
    #[test]
    fn case_folding_ascii() {
        let input = "CAFE";
        let output = "cafe";
        assert!(CaseFold::Full.casecmp(input, output));
        assert!(CaseFold::Full.casecmp(output, input));
        assert!(CaseFold::Ascii.casecmp(input, output));
        assert!(CaseFold::Ascii.casecmp(output, input));
        assert!(CaseFold::Turkic.casecmp(input, output));
        assert!(CaseFold::Turkic.casecmp(output, input));
        assert!(CaseFold::Lithuanian.casecmp(input, output));
        assert!(CaseFold::Lithuanian.casecmp(output, input));
    }

    // https://tools.ietf.org/html/draft-josefsson-idn-test-vectors-00#section-4.3
    #[test]
    fn case_folding_8bit() {
        let input = "ß";
        let output = "ss";
        assert!(CaseFold::Full.casecmp(input, output));
        assert!(CaseFold::Full.casecmp(output, input));
        assert!(!CaseFold::Ascii.casecmp(input, output));
        assert!(!CaseFold::Ascii.casecmp(output, input));
        assert!(CaseFold::Turkic.casecmp(input, output));
        assert!(CaseFold::Turkic.casecmp(output, input));
        assert!(CaseFold::Lithuanian.casecmp(input, output));
        assert!(CaseFold::Lithuanian.casecmp(output, input));
    }

    // https://tools.ietf.org/html/draft-josefsson-idn-test-vectors-00#section-4.4
    #[test]
    fn case_folding_turkic_capital_i_with_dot() {
        let input = "İ";
        let output = "i";
        assert!(CaseFold::Turkic.casecmp(input, output));
        assert!(CaseFold::Turkic.casecmp(output, input));
    }

    // https://tools.ietf.org/html/draft-josefsson-idn-test-vectors-00#section-4.5
    //
    // Multibyte folding is not supported.
    #[test]
    #[should_panic]
    fn case_folding_multibyte() {
        let input = "Ńͺ";
        let output = "ń ι";
        assert!(CaseFold::Full.casecmp(input, output));
        assert!(CaseFold::Full.casecmp(output, input));
        assert!(!CaseFold::Ascii.casecmp(input, output));
        assert!(!CaseFold::Ascii.casecmp(output, input));
        assert!(CaseFold::Turkic.casecmp(input, output));
        assert!(CaseFold::Turkic.casecmp(output, input));
        assert!(CaseFold::Lithuanian.casecmp(input, output));
        assert!(CaseFold::Lithuanian.casecmp(output, input));
    }

    // https://tools.ietf.org/html/draft-josefsson-idn-test-vectors-00#section-4.6
    //
    // These folding operations are not supported.
    #[test]
    #[should_panic]
    fn case_folding() {
        let input = "℡㏆𝞻";
        let output = "telc∕kgσ";
        assert!(CaseFold::Full.casecmp(input, output));
        assert!(CaseFold::Full.casecmp(output, input));
        assert!(!CaseFold::Ascii.casecmp(input, output));
        assert!(!CaseFold::Ascii.casecmp(output, input));
        assert!(CaseFold::Turkic.casecmp(input, output));
        assert!(CaseFold::Turkic.casecmp(output, input));
        assert!(CaseFold::Lithuanian.casecmp(input, output));
        assert!(CaseFold::Lithuanian.casecmp(output, input));
    }
}
