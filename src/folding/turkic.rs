use core::cmp::Ordering;

use crate::folding::mapping::{lookup, Mode};

/// Compare two strings with Full Unicode case folding for Turkic languages.
///
/// This function is implemented with a lookup table generated from Unicode case
/// folding tables.
///
/// # Examples
///
/// ```
/// # use core::cmp::Ordering;
/// # use focaccia::unicode_full_turkic_casecmp;
/// assert!(matches!(unicode_full_turkic_casecmp("İstanbul", "istanbul"), Ordering::Equal));
/// assert!(!matches!(unicode_full_turkic_casecmp("İstanbul", "Istanbul"), Ordering::Equal));
/// ```
///
/// # Examples – Full
///
/// Turkic case folding is largely compatible with full Unicode case folding.
///
/// ```
/// # use core::cmp::Ordering;
/// # use focaccia::unicode_full_turkic_casecmp;
/// assert_eq!(unicode_full_turkic_casecmp("MASSE", "Maße"), Ordering::Equal);
/// assert_eq!(unicode_full_turkic_casecmp("São Paulo", "Sao Paulo"), Ordering::Greater);
/// ```
#[inline]
#[must_use]
pub fn casecmp(left: &str, right: &str) -> Ordering {
    let left = left.chars().flat_map(|c| lookup(c, Mode::Turkic));
    let right = right.chars().flat_map(|c| lookup(c, Mode::Turkic));
    left.cmp(right)
}

/// Check two strings for equality with Full Unicode case folding for Turkic
/// languages.
///
/// This function is implemented with a lookup table generated from Unicode case
/// folding tables.
///
/// # Examples
///
/// ```
/// # use focaccia::unicode_full_turkic_case_eq;
/// assert!(unicode_full_turkic_case_eq("İstanbul", "istanbul"));
/// assert!(!unicode_full_turkic_case_eq("İstanbul", "Istanbul"));
/// ```
///
/// # Examples – Full
///
/// Turkic case folding is largely compatible with full Unicode case folding.
///
/// ```
/// # use focaccia::unicode_full_turkic_case_eq;
/// assert!(unicode_full_turkic_case_eq("MASSE", "Maße"));
/// assert!(!unicode_full_turkic_case_eq("São Paulo", "Sao Paulo"));
/// ```
#[inline]
#[must_use]
pub fn eq(left: &str, right: &str) -> bool {
    let left = left.chars().flat_map(|c| lookup(c, Mode::Turkic));
    let right = right.chars().flat_map(|c| lookup(c, Mode::Turkic));
    left.eq(right)
}

#[cfg(test)]
mod tests {
    use super::{casecmp, eq};
    use core::cmp::Ordering;

    #[test]
    fn compares_symbols_without_regard_to_case() {
        assert!(!eq("abcdef", "abcde"));
        assert!(eq("aBcDeF", "abcdef"));
        assert!(!eq("abcdef", "abcdefg"));
        assert!(eq("abcdef", "ABCDEF"));

        assert!(matches!(casecmp("abcdef", "abcde"), Ordering::Greater));
        assert!(matches!(casecmp("aBcDeF", "abcdef"), Ordering::Equal));
        assert!(matches!(casecmp("abcdef", "abcdefg"), Ordering::Less));
        assert!(matches!(casecmp("abcdef", "ABCDEF"), Ordering::Equal));

        assert_eq!(casecmp("abcdef", "abcde") as i32, 1);
        assert_eq!(casecmp("aBcDeF", "abcdef") as i32, 0);
        assert_eq!(casecmp("abcdef", "abcdefg") as i32, -1);
        assert_eq!(casecmp("abcdef", "ABCDEF") as i32, 0);
        assert_eq!(casecmp("abcdef", "abcde") as i32, 1);
    }

    #[test]
    fn doesent_consider_non_ascii_chars_equal_that_arent() {
        // -- UTF-8 --
        let upper_a_tilde = "Ã";
        let lower_a_tilde = "ã";
        let upper_a_umlaut = "Ä";
        let lower_a_umlaut = "ä";

        // From `spec/core/symbol/casecmp_spec.rb`:
        //
        // ```ruby
        // lower_a_tilde.casecmp?(lower_a_umlaut).should_not == true
        // lower_a_umlaut.casecmp?(lower_a_tilde).should_not == true
        // upper_a_tilde.casecmp?(upper_a_umlaut).should_not == true
        // upper_a_umlaut.casecmp?(upper_a_tilde).should_not == true
        // ```
        assert!(!eq(lower_a_tilde, lower_a_umlaut));
        assert!(!eq(lower_a_umlaut, lower_a_tilde));
        assert!(!eq(upper_a_tilde, upper_a_umlaut));
        assert!(!eq(upper_a_umlaut, upper_a_tilde));

        assert!(!matches!(
            casecmp(lower_a_tilde, lower_a_umlaut),
            Ordering::Equal
        ));
        assert!(!matches!(
            casecmp(lower_a_umlaut, lower_a_tilde),
            Ordering::Equal
        ));
        assert!(!matches!(
            casecmp(upper_a_tilde, upper_a_umlaut),
            Ordering::Equal
        ));
        assert!(!matches!(
            casecmp(upper_a_umlaut, upper_a_tilde),
            Ordering::Equal
        ));
    }

    #[test]
    fn does_case_mapping_for_unicode_chars() {
        // -- UTF-8 --
        let upper_a_tilde = "Ã";
        let lower_a_tilde = "ã";
        let upper_a_umlaut = "Ä";
        let lower_a_umlaut = "ä";

        // From `spec/core/symbol/casecmp_spec.rb`:
        //
        // ```ruby
        // upper_a_tilde.casecmp?(lower_a_tilde).should == true
        // upper_a_umlaut.casecmp?(lower_a_umlaut).should == true
        // lower_a_tilde.casecmp?(upper_a_tilde).should == true
        // lower_a_umlaut.casecmp?(upper_a_umlaut).should == true
        // ```
        assert!(eq(upper_a_tilde, lower_a_tilde));
        assert!(eq(upper_a_umlaut, lower_a_umlaut));
        assert!(eq(lower_a_tilde, upper_a_tilde));
        assert!(eq(lower_a_umlaut, upper_a_umlaut));

        assert!(matches!(
            casecmp(upper_a_tilde, lower_a_tilde),
            Ordering::Equal
        ));
        assert!(matches!(
            casecmp(upper_a_umlaut, lower_a_umlaut),
            Ordering::Equal
        ));
        assert!(matches!(
            casecmp(lower_a_tilde, upper_a_tilde),
            Ordering::Equal
        ));
        assert!(matches!(
            casecmp(lower_a_umlaut, upper_a_umlaut),
            Ordering::Equal
        ));
    }

    #[test]
    fn does_case_mapping_for_turkic_unicode_chars() {
        // -- UTF-8 --
        let upper_dotless_i = "I";
        let lower_dotless_i = "ı";
        let upper_dotted_i = "İ";
        let lower_dotted_i = "i";

        assert!(eq(upper_dotless_i, lower_dotless_i));
        assert!(eq(upper_dotted_i, lower_dotted_i));
        assert!(eq(lower_dotless_i, upper_dotless_i));
        assert!(eq(lower_dotted_i, upper_dotted_i));

        assert!(matches!(
            casecmp(upper_dotless_i, lower_dotless_i),
            Ordering::Equal
        ));
        assert!(matches!(
            casecmp(upper_dotted_i, lower_dotted_i),
            Ordering::Equal
        ));
        assert!(matches!(
            casecmp(lower_dotless_i, upper_dotless_i),
            Ordering::Equal
        ));
        assert!(matches!(
            casecmp(lower_dotted_i, upper_dotted_i),
            Ordering::Equal
        ));

        assert!(!eq(upper_dotless_i, upper_dotted_i));
        assert!(!eq(upper_dotless_i, lower_dotted_i));
        assert!(!eq(lower_dotless_i, upper_dotted_i));
        assert!(!eq(lower_dotless_i, lower_dotted_i));
        assert!(!eq(upper_dotted_i, upper_dotless_i));
        assert!(!eq(upper_dotted_i, lower_dotless_i));
        assert!(!eq(lower_dotted_i, upper_dotless_i));
        assert!(!eq(lower_dotted_i, lower_dotless_i));

        assert!(!matches!(
            casecmp(upper_dotless_i, upper_dotted_i),
            Ordering::Equal
        ));
        assert!(!matches!(
            casecmp(upper_dotless_i, lower_dotted_i),
            Ordering::Equal
        ));
        assert!(!matches!(
            casecmp(lower_dotless_i, upper_dotted_i),
            Ordering::Equal
        ));
        assert!(!matches!(
            casecmp(lower_dotless_i, lower_dotted_i),
            Ordering::Equal
        ));
        assert!(!matches!(
            casecmp(upper_dotted_i, upper_dotless_i),
            Ordering::Equal
        ));
        assert!(!matches!(
            casecmp(upper_dotted_i, lower_dotless_i),
            Ordering::Equal
        ));
        assert!(!matches!(
            casecmp(lower_dotted_i, upper_dotless_i),
            Ordering::Equal
        ));
        assert!(!matches!(
            casecmp(lower_dotted_i, lower_dotless_i),
            Ordering::Equal
        ));
    }
}
