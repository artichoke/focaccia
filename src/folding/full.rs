use crate::folding::mapping::{lookup, Mode};

/// Compare two strings with Full Unicode case folding.
///
/// This function is implemented with a lookup table generated from Unicode case
/// folding tables.
#[must_use]
#[allow(clippy::match_same_arms)]
pub fn casecmp(left: &str, right: &str) -> bool {
    let left = left.chars().flat_map(|c| lookup(c, Mode::Full));
    let right = right.chars().flat_map(|c| lookup(c, Mode::Full));
    left.eq(right)
}

#[cfg(test)]
mod tests {
    use super::casecmp;

    #[test]
    fn compares_symbols_without_regard_to_case() {
        assert!(!casecmp("abcdef", "abcde"));
        assert!(casecmp("aBcDeF", "abcdef"));
        assert!(!casecmp("abcdef", "abcdefg"));
        assert!(casecmp("abcdef", "ABCDEF"));
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
        assert!(!casecmp(lower_a_tilde, lower_a_umlaut));
        assert!(!casecmp(lower_a_umlaut, lower_a_tilde));
        assert!(!casecmp(upper_a_tilde, upper_a_umlaut));
        assert!(!casecmp(upper_a_umlaut, upper_a_tilde));
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
        assert!(casecmp(upper_a_tilde, lower_a_tilde));
        assert!(casecmp(upper_a_umlaut, lower_a_umlaut));
        assert!(casecmp(lower_a_tilde, upper_a_tilde));
        assert!(casecmp(lower_a_umlaut, upper_a_umlaut));
    }
}
