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

//! Case folding comparisons for byte content resolved from `Symbol`s.

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
/// The `Fold` enum intends to support the [folding strategies available in
/// Ruby].
///
/// [folding strategies available in Ruby]: https://ruby-doc.org/core-2.6.3/String.html#method-i-downcase
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Fold {
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

impl Default for Fold {
    /// Default to full Unicode case folding.
    ///
    /// See [`Fold::Full`].
    #[inline]
    fn default() -> Self {
        Self::Full
    }
}

impl Fold {
    /// Make a case-insensitive string comparison based on the dispatching
    /// folding strategy.
    ///
    /// Return `true` if the given strings match when folding case. For example,
    /// when using `Fold::Full`, `ß` is considered equal to `ss`. The
    /// differences between the folding strategies are documented on the
    /// variants of the [`Fold`] enum.
    ///
    /// This function is a wrapper around the underlying scheme-specific
    /// functions.
    #[inline]
    #[must_use]
    pub fn casecmp(&self, left: &str, right: &str) -> bool {
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
pub struct NoSuchFoldingScheme(());

impl NoSuchFoldingScheme {
    /// Construct a new [`NoSuchFoldingScheme`] error.
    pub fn new() -> Self {
        Self::default()
    }
}

impl fmt::Display for NoSuchFoldingScheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("No such Unicode case folding scheme exists")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for NoSuchFoldingScheme {}

impl TryFrom<Option<&str>> for Fold {
    type Error = NoSuchFoldingScheme;

    fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
        match value {
            None => Ok(Self::Full),
            Some("ascii") => Ok(Self::Ascii),
            Some("turkic") => Ok(Self::Turkic),
            Some("lithuanian") => Ok(Self::Lithuanian),
            _ => Err(NoSuchFoldingScheme::new()),
        }
    }
}

impl TryFrom<Option<&[u8]>> for Fold {
    type Error = NoSuchFoldingScheme;

    fn try_from(value: Option<&[u8]>) -> Result<Self, Self::Error> {
        match value {
            None => Ok(Self::Full),
            Some(scheme) if scheme == b"ascii" => Ok(Self::Ascii),
            Some(scheme) if scheme == b"turkic" => Ok(Self::Turkic),
            Some(scheme) if scheme == b"lithuanian" => Ok(Self::Lithuanian),
            _ => Err(NoSuchFoldingScheme::new()),
        }
    }
}

// Tests using IDN case folding test vectors:
#[cfg(test)]
mod tests {
    use crate::Fold;

    // https://tools.ietf.org/html/draft-josefsson-idn-test-vectors-00#section-4.2
    #[test]
    fn case_folding_ascii() {
        let input = "CAFE";
        let output = "cafe";
        assert!(Fold::Full.casecmp(input, output));
        assert!(Fold::Full.casecmp(output, input));
        assert!(Fold::Ascii.casecmp(input, output));
        assert!(Fold::Ascii.casecmp(output, input));
        assert!(Fold::Turkic.casecmp(input, output));
        assert!(Fold::Turkic.casecmp(output, input));
        assert!(Fold::Lithuanian.casecmp(input, output));
        assert!(Fold::Lithuanian.casecmp(output, input));
    }

    // https://tools.ietf.org/html/draft-josefsson-idn-test-vectors-00#section-4.3
    #[test]
    fn case_folding_8bit() {
        let input = "ß";
        let output = "ss";
        assert!(Fold::Full.casecmp(input, output));
        assert!(Fold::Full.casecmp(output, input));
        assert!(!Fold::Ascii.casecmp(input, output));
        assert!(!Fold::Ascii.casecmp(output, input));
        assert!(Fold::Turkic.casecmp(input, output));
        assert!(Fold::Turkic.casecmp(output, input));
        assert!(Fold::Lithuanian.casecmp(input, output));
        assert!(Fold::Lithuanian.casecmp(output, input));
    }

    // https://tools.ietf.org/html/draft-josefsson-idn-test-vectors-00#section-4.4
    #[test]
    fn case_folding_turkic_capital_i_with_dot() {
        let input = "İ";
        let output = "i";
        assert!(Fold::Turkic.casecmp(input, output));
        assert!(Fold::Turkic.casecmp(output, input));
    }

    // https://tools.ietf.org/html/draft-josefsson-idn-test-vectors-00#section-4.5
    //
    // Multibyte folding is not supported.
    #[test]
    #[should_panic]
    fn case_folding_multibyte() {
        let input = "Ńͺ";
        let output = "ń ι";
        assert!(Fold::Full.casecmp(input, output));
        assert!(Fold::Full.casecmp(output, input));
        assert!(!Fold::Ascii.casecmp(input, output));
        assert!(!Fold::Ascii.casecmp(output, input));
        assert!(Fold::Turkic.casecmp(input, output));
        assert!(Fold::Turkic.casecmp(output, input));
        assert!(Fold::Lithuanian.casecmp(input, output));
        assert!(Fold::Lithuanian.casecmp(output, input));
    }

    // https://tools.ietf.org/html/draft-josefsson-idn-test-vectors-00#section-4.6
    //
    // These folding operations are not supported.
    #[test]
    #[should_panic]
    fn case_folding() {
        let input = "℡㏆𝞻";
        let output = "telc∕kgσ";
        assert!(Fold::Full.casecmp(input, output));
        assert!(Fold::Full.casecmp(output, input));
        assert!(!Fold::Ascii.casecmp(input, output));
        assert!(!Fold::Ascii.casecmp(output, input));
        assert!(Fold::Turkic.casecmp(input, output));
        assert!(Fold::Turkic.casecmp(output, input));
        assert!(Fold::Lithuanian.casecmp(input, output));
        assert!(Fold::Lithuanian.casecmp(output, input));
    }
}
