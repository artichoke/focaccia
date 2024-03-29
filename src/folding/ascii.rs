use core::cmp::Ordering;

/// Compare two byte strings with ASCII case folding.
///
/// This function is implemented with ASCII folding functions in Rust `core`.
///
/// # Examples
///
/// ```
/// # use core::cmp::Ordering;
/// # use focaccia::{ascii_casecmp, ascii_case_eq};
/// assert_eq!(ascii_casecmp(b"Artichoke Ruby", b"artichoke ruby"), Ordering::Equal);
/// assert_eq!(ascii_casecmp(b"artichoke ruby", b"Artichoke"), Ordering::Greater);
/// ```
#[inline]
#[must_use]
pub fn casecmp(left: &[u8], right: &[u8]) -> Ordering {
    let left = left.iter().map(u8::to_ascii_lowercase);
    let right = right.iter().map(u8::to_ascii_lowercase);
    left.cmp(right)
}

/// Check two byte strings for equality with ASCII case folding.
///
/// This function is implemented with ASCII folding functions in Rust `core`.
///
/// # Examples
///
/// ```
/// # use focaccia::{ascii_casecmp, ascii_case_eq};
/// assert!(ascii_case_eq(b"Artichoke Ruby", b"artichoke ruby"));
/// assert!(!ascii_case_eq(b"artichoke ruby", b"Artichoke"));
/// ```
#[inline]
#[must_use]
pub fn case_eq(left: &[u8], right: &[u8]) -> bool {
    left.eq_ignore_ascii_case(right)
}

#[cfg(test)]
mod tests {
    use core::cmp::Ordering;

    use super::{case_eq, casecmp};

    #[test]
    fn empty_string() {
        assert!(case_eq(b"", b""));
        assert_eq!(casecmp(b"", b""), Ordering::Equal);

        assert!(!case_eq(b"", b"rake"));
        assert_eq!(casecmp(b"", b"rake"), Ordering::Less);

        assert!(!case_eq(b"rake", b""));
        assert_eq!(casecmp(b"rake", b""), Ordering::Greater);
    }

    #[test]
    fn unicode_replacement_character() {
        assert!(case_eq("\u{FFFD}".as_bytes(), "\u{FFFD}".as_bytes()));
        assert_eq!(
            casecmp("\u{FFFD}".as_bytes(), "\u{FFFD}".as_bytes()),
            Ordering::Equal
        );

        assert_eq!(
            casecmp("\u{FFFD}".as_bytes(), "\u{FFFD}yam".as_bytes()),
            Ordering::Less
        );
        assert_eq!(
            casecmp("\u{FFFD}yam".as_bytes(), "\u{FFFD}".as_bytes()),
            Ordering::Greater
        );
    }

    #[test]
    fn compares_symbols_without_regard_to_case() {
        assert!(!case_eq(b"abcdef", b"abcde"));
        assert!(case_eq(b"aBcDeF", b"abcdef"));
        assert!(!case_eq(b"abcdef", b"abcdefg"));
        assert!(case_eq(b"abcdef", b"ABCDEF"));

        assert_eq!(casecmp(b"abcdef", b"abcde"), Ordering::Greater);
        assert_eq!(casecmp(b"aBcDeF", b"abcdef"), Ordering::Equal);
        assert_eq!(casecmp(b"abcdef", b"abcdefg"), Ordering::Less);
        assert_eq!(casecmp(b"abcdef", b"ABCDEF"), Ordering::Equal);

        assert_eq!(casecmp(b"abcdef", b"abcde") as i32, 1);
        assert_eq!(casecmp(b"aBcDeF", b"abcdef") as i32, 0);
        assert_eq!(casecmp(b"abcdef", b"abcdefg") as i32, -1);
        assert_eq!(casecmp(b"abcdef", b"ABCDEF") as i32, 0);
        assert_eq!(casecmp(b"abcdef", b"abcde") as i32, 1);
    }

    #[test]
    fn non_ascii_chars_that_are_not_fold_eq_are_not_eq() {
        // -- Latin-1 --
        let upper_a_tilde = b"\xC3";
        let upper_a_umlaut = b"\xC4";
        let lower_a_tilde = b"\xE3";
        let lower_a_umlaut = b"\xE4";

        // From `spec/core/symbol/casecmp_spec.rb`:
        //
        // ```ruby
        // lower_a_tilde.casecmp(lower_a_umlaut).should_not == 0
        // lower_a_umlaut.casecmp(lower_a_tilde).should_not == 0
        // upper_a_tilde.casecmp(upper_a_umlaut).should_not == 0
        // upper_a_umlaut.casecmp(upper_a_tilde).should_not == 0
        // ```
        assert!(!case_eq(lower_a_tilde, lower_a_umlaut));
        assert!(!case_eq(lower_a_umlaut, lower_a_tilde));
        assert!(!case_eq(upper_a_tilde, upper_a_umlaut));
        assert!(!case_eq(upper_a_umlaut, upper_a_tilde));

        assert_ne!(casecmp(lower_a_tilde, lower_a_umlaut), Ordering::Equal);
        assert_ne!(casecmp(lower_a_umlaut, lower_a_tilde), Ordering::Equal);
        assert_ne!(casecmp(upper_a_tilde, upper_a_umlaut), Ordering::Equal);
        assert_ne!(casecmp(upper_a_umlaut, upper_a_tilde), Ordering::Equal);

        // -- UTF-8 --
        let upper_a_tilde = "Ã".as_bytes();
        let lower_a_tilde = "ã".as_bytes();
        let upper_a_umlaut = "Ä".as_bytes();
        let lower_a_umlaut = "ä".as_bytes();

        // From `spec/core/symbol/casecmp_spec.rb`:
        //
        // ```ruby
        // lower_a_tilde.casecmp(lower_a_umlaut).should_not == 0
        // lower_a_umlaut.casecmp(lower_a_tilde).should_not == 0
        // upper_a_tilde.casecmp(upper_a_umlaut).should_not == 0
        // upper_a_umlaut.casecmp(upper_a_tilde).should_not == 0
        // ```
        assert!(!case_eq(lower_a_tilde, lower_a_umlaut));
        assert!(!case_eq(lower_a_umlaut, lower_a_tilde));
        assert!(!case_eq(upper_a_tilde, upper_a_umlaut));
        assert!(!case_eq(upper_a_umlaut, upper_a_tilde));

        assert_ne!(casecmp(lower_a_tilde, lower_a_umlaut), Ordering::Equal);
        assert_ne!(casecmp(lower_a_umlaut, lower_a_tilde), Ordering::Equal);
        assert_ne!(casecmp(upper_a_tilde, upper_a_umlaut), Ordering::Equal);
        assert_ne!(casecmp(upper_a_umlaut, upper_a_tilde), Ordering::Equal);
    }

    #[test]
    fn doesent_do_case_mapping_for_non_ascii_chars() {
        // -- Latin-1 --
        let upper_a_tilde = b"\xC3";
        let upper_a_umlaut = b"\xC4";
        let lower_a_tilde = b"\xE3";
        let lower_a_umlaut = b"\xE4";

        // From `spec/core/symbol/casecmp_spec.rb`:
        //
        // ```ruby
        // upper_a_tilde.casecmp(lower_a_tilde).should == -1
        // upper_a_umlaut.casecmp(lower_a_umlaut).should == -1
        // lower_a_tilde.casecmp(upper_a_tilde).should == 1
        // lower_a_umlaut.casecmp(upper_a_umlaut).should == 1
        // ```
        assert!(!case_eq(upper_a_tilde, lower_a_tilde));
        assert!(!case_eq(upper_a_umlaut, lower_a_umlaut));
        assert!(!case_eq(lower_a_tilde, upper_a_tilde));
        assert!(!case_eq(lower_a_umlaut, upper_a_umlaut));

        assert_eq!(casecmp(upper_a_tilde, lower_a_tilde), Ordering::Less);
        assert_eq!(casecmp(upper_a_umlaut, lower_a_umlaut), Ordering::Less);
        assert_eq!(casecmp(lower_a_tilde, upper_a_tilde), Ordering::Greater);
        assert_eq!(casecmp(lower_a_umlaut, upper_a_umlaut), Ordering::Greater);

        assert_eq!(casecmp(upper_a_tilde, lower_a_tilde) as i32, -1);
        assert_eq!(casecmp(upper_a_umlaut, lower_a_umlaut) as i32, -1);
        assert_eq!(casecmp(lower_a_tilde, upper_a_tilde) as i32, 1);
        assert_eq!(casecmp(lower_a_umlaut, upper_a_umlaut) as i32, 1);

        // -- UTF-8 --
        let upper_a_tilde = "Ã".as_bytes();
        let lower_a_tilde = "ã".as_bytes();
        let upper_a_umlaut = "Ä".as_bytes();
        let lower_a_umlaut = "ä".as_bytes();

        // From `spec/core/symbol/casecmp_spec.rb`:
        //
        // ```ruby
        // upper_a_tilde.casecmp(lower_a_tilde).should == -1
        // upper_a_umlaut.casecmp(lower_a_umlaut).should == -1
        // lower_a_tilde.casecmp(upper_a_tilde).should == 1
        // lower_a_umlaut.casecmp(upper_a_umlaut).should == 1
        // ```
        assert!(!case_eq(upper_a_tilde, lower_a_tilde));
        assert!(!case_eq(upper_a_umlaut, lower_a_umlaut));
        assert!(!case_eq(lower_a_tilde, upper_a_tilde));
        assert!(!case_eq(lower_a_umlaut, upper_a_umlaut));

        assert_eq!(casecmp(upper_a_tilde, lower_a_tilde), Ordering::Less);
        assert_eq!(casecmp(upper_a_umlaut, lower_a_umlaut), Ordering::Less);
        assert_eq!(casecmp(lower_a_tilde, upper_a_tilde), Ordering::Greater);
        assert_eq!(casecmp(lower_a_umlaut, upper_a_umlaut), Ordering::Greater);

        assert_eq!(casecmp(upper_a_tilde, lower_a_tilde) as i32, -1);
        assert_eq!(casecmp(upper_a_umlaut, lower_a_umlaut) as i32, -1);
        assert_eq!(casecmp(lower_a_tilde, upper_a_tilde) as i32, 1);
        assert_eq!(casecmp(lower_a_umlaut, upper_a_umlaut) as i32, 1);
    }

    #[test]
    fn exhaustive() {
        let lower = 'a'..='z';
        let upper = 'A'..='Z';
        let mut l_buf = [0; 4];
        let mut r_buf = [0; 4];
        for (left, right) in lower.zip(upper) {
            let left = left.encode_utf8(&mut l_buf);
            let right = right.encode_utf8(&mut r_buf);
            assert_eq!(casecmp(left.as_bytes(), right.as_bytes()), Ordering::Equal);
            assert_eq!(casecmp(right.as_bytes(), left.as_bytes()), Ordering::Equal);
        }
    }
}
