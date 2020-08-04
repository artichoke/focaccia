#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![cfg_attr(test, allow(clippy::non_ascii_literal))]
#![cfg_attr(test, allow(clippy::shadow_unrelated))]
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
//! # use core::cmp::Ordering;
//! # use focaccia::CaseFold;
//! let fold = CaseFold::Full;
//! assert_eq!(fold.casecmp("MASSE", "Maße"), Ordering::Equal);
//! assert_eq!(fold.casecmp("São Paulo", "Sao Paulo"), Ordering::Greater);
//!
//! assert!(fold.case_eq("MASSE", "Maße"));
//! assert!(!fold.case_eq("São Paulo", "Sao Paulo"));
//! ```
//!
//! For text known to be ASCII, Focaccia can make a more performant comparison
//! check:
//!
//! ```
//! # use core::cmp::Ordering;
//! # use focaccia::CaseFold;
//! let fold = CaseFold::Ascii;
//! assert_eq!(fold.casecmp("Crate: focaccia", "Crate: FOCACCIA"), Ordering::Equal);
//! assert_eq!(fold.casecmp("Fabled", "failed"), Ordering::Less);
//!
//! assert!(fold.case_eq("Crate: focaccia", "Crate: FOCACCIA"));
//! assert!(!fold.case_eq("Fabled", "failed"));
//! ```
//!
//! ASCII case comparison can be checked on a byte slice:
//!
//! ```
//! # use core::cmp::Ordering;
//! # use focaccia::{ascii_casecmp, ascii_case_eq};
//! assert_eq!(ascii_casecmp(b"Artichoke Ruby", b"artichoke ruby"), Ordering::Equal);
//! assert!(ascii_case_eq(b"Artichoke Ruby", b"artichoke ruby"));
//! ```
//!
//! Turkic case folding is similar to full case folding with additional mappings
//! for [dotted and dotless I]:
//!
//! ```
//! # use core::cmp::Ordering;
//! # use focaccia::CaseFold;
//! let fold = CaseFold::Turkic;
//! assert!(matches!(fold.casecmp("İstanbul", "istanbul"), Ordering::Equal));
//! assert!(!matches!(fold.casecmp("İstanbul", "Istanbul"), Ordering::Equal));
//!
//! assert!(fold.case_eq("İstanbul", "istanbul"));
//! assert!(!fold.case_eq("İstanbul", "Istanbul"));
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

use core::cmp::Ordering;
use core::convert::TryFrom;
use core::fmt;

mod folding;

pub use folding::{
    ascii_case_eq, ascii_casecmp, unicode_full_case_eq, unicode_full_casecmp,
    unicode_full_turkic_case_eq, unicode_full_turkic_casecmp,
};

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
/// # Examples
///
/// For Unicode text, the default folding schem is [`Full`](CaseFold::Full). To
/// make an equality comparison:
///
/// ```
/// # use core::cmp::Ordering;
/// # use focaccia::CaseFold;
/// let fold = CaseFold::Full;
/// assert_eq!(fold.casecmp("MASSE", "Maße"), Ordering::Equal);
/// assert_eq!(fold.casecmp("São Paulo", "Sao Paulo"), Ordering::Greater);
///
/// assert!(fold.case_eq("MASSE", "Maße"));
/// assert!(!fold.case_eq("São Paulo", "Sao Paulo"));
/// ```
///
/// For text known to be ASCII, Focaccia can make a more performant comparison
/// check:
///
/// ```
/// # use core::cmp::Ordering;
/// # use focaccia::CaseFold;
/// let fold = CaseFold::Ascii;
/// assert_eq!(fold.casecmp("Crate: focaccia", "Crate: FOCACCIA"), Ordering::Equal);
/// assert_eq!(fold.casecmp("Fabled", "failed"), Ordering::Less);
///
/// assert!(fold.case_eq("Crate: focaccia", "Crate: FOCACCIA"));
/// assert!(!fold.case_eq("Fabled", "failed"));
/// ```
///
/// Turkic case folding is similar to full case folding with additional mappings
/// for [dotted and dotless I]:
///
/// ```
/// # use core::cmp::Ordering;
/// # use focaccia::CaseFold;
/// let fold = CaseFold::Turkic;
/// assert!(matches!(fold.casecmp("İstanbul", "istanbul"), Ordering::Equal));
/// assert!(!matches!(fold.casecmp("İstanbul", "Istanbul"), Ordering::Equal));
///
/// assert!(fold.case_eq("İstanbul", "istanbul"));
/// assert!(!fold.case_eq("İstanbul", "Istanbul"));
/// ```
///
/// [folding strategies available in Ruby]: https://ruby-doc.org/core-2.6.3/String.html#method-i-downcase
/// [dotted and dotless I]: https://en.wikipedia.org/wiki/Dotted_and_dotless_I
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
    ///
    /// # Examples
    ///
    /// ```
    /// # use focaccia::CaseFold;
    /// assert_eq!(CaseFold::default(), CaseFold::Full);
    ///
    /// assert!(CaseFold::default().case_eq("MASSE", "Maße"));
    /// assert!(!CaseFold::default().case_eq("São Paulo", "Sao Paulo"));
    /// ```
    #[inline]
    fn default() -> Self {
        Self::Full
    }
}

impl CaseFold {
    /// Construct a new full Unicode case folding.
    ///
    /// See [`CaseFold::Full`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use focaccia::CaseFold;
    /// assert_eq!(CaseFold::new(), CaseFold::Full);
    ///
    /// assert!(CaseFold::new().case_eq("MASSE", "Maße"));
    /// assert!(!CaseFold::new().case_eq("São Paulo", "Sao Paulo"));
    /// ```
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self::Full
    }

    /// Make a case-insensitive string comparison based on the dispatching
    /// folding strategy.
    ///
    /// Return `Ordering::Equal` if the given strings match when folding case.
    /// For example, when using `CaseFold::Full`, `ß` is considered equal to
    /// `ss`. The differences between the folding strategies are documented on
    /// the variants of the [`CaseFold`] enum.
    ///
    /// This function is a wrapper around the underlying scheme-specific
    /// functions.
    ///
    /// # Examples – Full case folding
    ///
    /// ```
    /// # use core::cmp::Ordering;
    /// # use focaccia::CaseFold;
    /// let fold = CaseFold::Full;
    /// assert_eq!(fold.casecmp("MASSE", "Maße"), Ordering::Equal);
    /// assert_eq!(fold.casecmp("São Paulo", "Sao Paulo"), Ordering::Greater);
    /// ```
    ///
    /// # Examples – ASCII case folding
    ///
    /// ```
    /// # use core::cmp::Ordering;
    /// # use focaccia::CaseFold;
    /// let fold = CaseFold::Ascii;
    /// assert_eq!(fold.casecmp("Crate: focaccia", "Crate: FOCACCIA"), Ordering::Equal);
    /// assert_eq!(fold.casecmp("Fabled", "failed"), Ordering::Less);
    /// ```
    ///
    /// # Examples – Turkic case folding
    ///
    /// ```
    /// # use core::cmp::Ordering;
    /// # use focaccia::CaseFold;
    /// let fold = CaseFold::Turkic;
    /// assert!(matches!(fold.casecmp("İstanbul", "istanbul"), Ordering::Equal));
    /// assert!(!matches!(fold.casecmp("İstanbul", "Istanbul"), Ordering::Equal));
    /// ```
    #[inline]
    #[must_use]
    pub fn casecmp(self, left: &str, right: &str) -> Ordering {
        match self {
            Self::Full | Self::Lithuanian => unicode_full_casecmp(left, right),
            Self::Ascii => ascii_casecmp(left.as_bytes(), right.as_bytes()),
            Self::Turkic => unicode_full_turkic_casecmp(left, right),
        }
    }

    /// Make a case-insensitive string equality check based on the dispatching
    /// folding strategy.
    ///
    /// Return `true` if the given strings match when folding case. For example,
    /// when using `CaseFold::Full`, `ß` is considered equal to `ss`. The
    /// differences between the folding strategies are documented on the
    /// variants of the [`CaseFold`] enum.
    ///
    /// This function is a wrapper around the underlying scheme-specific
    /// functions.
    ///
    /// # Examples – Full case folding
    ///
    /// ```
    /// # use focaccia::CaseFold;
    /// let fold = CaseFold::Full;
    /// assert!(fold.case_eq("MASSE", "Maße"));
    /// assert!(!fold.case_eq("São Paulo", "Sao Paulo"));
    /// ```
    ///
    /// # Examples – ASCII case folding
    ///
    /// ```
    /// # use focaccia::CaseFold;
    /// let fold = CaseFold::Ascii;
    /// assert!(fold.case_eq("Crate: focaccia", "Crate: FOCACCIA"));
    /// assert!(!fold.case_eq("Fabled", "failed"));
    /// ```
    ///
    /// # Examples – Turkic case folding
    ///
    /// ```
    /// # use focaccia::CaseFold;
    /// let fold = CaseFold::Turkic;
    /// assert!(fold.case_eq("İstanbul", "istanbul"));
    /// assert!(!fold.case_eq("İstanbul", "Istanbul"));
    /// ```
    #[inline]
    #[must_use]
    pub fn case_eq(self, left: &str, right: &str) -> bool {
        match self {
            Self::Full | Self::Lithuanian => unicode_full_case_eq(left, right),
            Self::Ascii => ascii_case_eq(left.as_bytes(), right.as_bytes()),
            Self::Turkic => unicode_full_turkic_case_eq(left, right),
        }
    }
}

/// Error type for returned when a folding scheme could not be resolved in a
/// [`TryFrom`] implementation.
///
/// When this crate's `std` feature is enabled, `NoSuchCaseFoldingScheme`
/// implements [`std::error::Error`].
///
/// # Examples
///
/// ```
/// # use core::convert::TryFrom;
/// # use focaccia::{CaseFold, NoSuchCaseFoldingScheme};
/// assert_eq!(CaseFold::try_from(None::<&str>), Ok(CaseFold::Full));
/// assert_eq!(CaseFold::try_from(Some("turkic")), Ok(CaseFold::Turkic));
/// assert_eq!(CaseFold::try_from(Some("xxx")), Err(NoSuchCaseFoldingScheme::new()));
/// ```
///
/// [`std::error::Error`]: https://doc.rust-lang.org/stable/std/error/trait.Error.html
#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NoSuchCaseFoldingScheme {
    _private: (),
}

impl NoSuchCaseFoldingScheme {
    /// Construct a new [`NoSuchCaseFoldingScheme`] error.
    ///
    /// # Examples
    ///
    /// ```
    /// # use focaccia::NoSuchCaseFoldingScheme;
    /// let err = NoSuchCaseFoldingScheme::new();
    /// ```
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self { _private: () }
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
        assert!(CaseFold::Full.case_eq(input, output));
        assert!(CaseFold::Full.case_eq(output, input));
        assert!(CaseFold::Ascii.case_eq(input, output));
        assert!(CaseFold::Ascii.case_eq(output, input));
        assert!(CaseFold::Turkic.case_eq(input, output));
        assert!(CaseFold::Turkic.case_eq(output, input));
        assert!(CaseFold::Lithuanian.case_eq(input, output));
        assert!(CaseFold::Lithuanian.case_eq(output, input));
    }

    // https://tools.ietf.org/html/draft-josefsson-idn-test-vectors-00#section-4.3
    #[test]
    fn case_folding_8bit() {
        let input = "ß";
        let output = "ss";
        assert!(CaseFold::Full.case_eq(input, output));
        assert!(CaseFold::Full.case_eq(output, input));
        assert!(!CaseFold::Ascii.case_eq(input, output));
        assert!(!CaseFold::Ascii.case_eq(output, input));
        assert!(CaseFold::Turkic.case_eq(input, output));
        assert!(CaseFold::Turkic.case_eq(output, input));
        assert!(CaseFold::Lithuanian.case_eq(input, output));
        assert!(CaseFold::Lithuanian.case_eq(output, input));
    }

    // https://tools.ietf.org/html/draft-josefsson-idn-test-vectors-00#section-4.4
    #[test]
    fn case_folding_turkic_capital_i_with_dot() {
        let input = "İ";
        let output = "i";
        assert!(CaseFold::Turkic.case_eq(input, output));
        assert!(CaseFold::Turkic.case_eq(output, input));
    }

    // https://tools.ietf.org/html/draft-josefsson-idn-test-vectors-00#section-4.5
    //
    // Multibyte folding is not supported.
    #[test]
    #[should_panic]
    fn case_folding_multibyte() {
        let input = "Ńͺ";
        let output = "ń ι";
        assert!(CaseFold::Full.case_eq(input, output));
        assert!(CaseFold::Full.case_eq(output, input));
        assert!(!CaseFold::Ascii.case_eq(input, output));
        assert!(!CaseFold::Ascii.case_eq(output, input));
        assert!(CaseFold::Turkic.case_eq(input, output));
        assert!(CaseFold::Turkic.case_eq(output, input));
        assert!(CaseFold::Lithuanian.case_eq(input, output));
        assert!(CaseFold::Lithuanian.case_eq(output, input));
    }

    // https://tools.ietf.org/html/draft-josefsson-idn-test-vectors-00#section-4.6
    //
    // These folding operations are not supported.
    #[test]
    #[should_panic]
    fn case_folding() {
        let input = "℡㏆𝞻";
        let output = "telc∕kgσ";
        assert!(CaseFold::Full.case_eq(input, output));
        assert!(CaseFold::Full.case_eq(output, input));
        assert!(!CaseFold::Ascii.case_eq(input, output));
        assert!(!CaseFold::Ascii.case_eq(output, input));
        assert!(CaseFold::Turkic.case_eq(input, output));
        assert!(CaseFold::Turkic.case_eq(output, input));
        assert!(CaseFold::Lithuanian.case_eq(input, output));
        assert!(CaseFold::Lithuanian.case_eq(output, input));
    }
}
