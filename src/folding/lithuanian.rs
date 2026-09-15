use core::cmp::Ordering;

use super::full;

/// Compare two strings with Ruby-compatible Lithuanian case folding.
///
/// This strategy uses full Unicode case folding, matching Ruby's current
/// Lithuanian behavior. It does not apply the context-dependent Lithuanian
/// mappings from `SpecialCasing.txt`, such as inserting a dot above accented I
/// or J. No Unicode normalization is performed.
///
/// See [Ruby's case mapping options](https://github.com/ruby/ruby/blob/v3_4_10/doc/case_mapping.rdoc#case-mapping-options).
///
/// # Examples
///
/// ```
/// # use core::cmp::Ordering;
/// # use focaccia::unicode_full_lithuanian_casecmp;
/// assert_eq!(unicode_full_lithuanian_casecmp("I\u{301}", "i\u{301}"), Ordering::Equal);
/// assert_eq!(unicode_full_lithuanian_casecmp("I\u{301}", "i\u{307}\u{301}"), Ordering::Less);
/// ```
#[inline]
#[must_use]
pub fn casecmp(left: &str, right: &str) -> Ordering {
    full::casecmp(left, right)
}

/// Check two strings for equality with Ruby-compatible Lithuanian case folding.
///
/// This strategy uses full Unicode case folding, matching Ruby's current
/// Lithuanian behavior. It does not apply the context-dependent Lithuanian
/// mappings from `SpecialCasing.txt`, such as inserting a dot above accented I
/// or J. No Unicode normalization is performed.
///
/// See [Ruby's case mapping options](https://github.com/ruby/ruby/blob/v3_4_10/doc/case_mapping.rdoc#case-mapping-options).
///
/// # Examples
///
/// ```
/// # use focaccia::unicode_full_lithuanian_case_eq;
/// assert!(unicode_full_lithuanian_case_eq("I\u{301}", "i\u{301}"));
/// assert!(!unicode_full_lithuanian_case_eq("I\u{301}", "i\u{307}\u{301}"));
/// assert!(unicode_full_lithuanian_case_eq("MASSE", "Maße"));
/// ```
#[inline]
#[must_use]
pub fn case_eq(left: &str, right: &str) -> bool {
    full::case_eq(left, right)
}
