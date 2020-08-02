use core::cmp::Ordering;

use crate::folding::mapping::{lookup, Mode};

/// Compare two strings with Full Unicode case folding.
///
/// This function is implemented with a lookup table generated from Unicode case
/// folding tables.
#[inline]
#[must_use]
pub fn casecmp(left: &str, right: &str) -> Ordering {
    let left = left.chars().flat_map(|c| lookup(c, Mode::Full));
    let right = right.chars().flat_map(|c| lookup(c, Mode::Full));
    left.cmp(right)
}

/// Check two strings for equality with Full Unicode case folding.
///
/// This function is implemented with a lookup table generated from Unicode case
/// folding tables.
#[inline]
#[must_use]
pub fn eq(left: &str, right: &str) -> bool {
    let left = left.chars().flat_map(|c| lookup(c, Mode::Full));
    let right = right.chars().flat_map(|c| lookup(c, Mode::Full));
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
}
