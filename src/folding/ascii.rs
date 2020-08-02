use core::cmp::Ordering;

/// Compare two bytestrings with ASCII case folding.
///
/// This function is implemented with ASCII folding functions in Rust `core`.
#[inline]
#[must_use]
pub fn casecmp(left: &[u8], right: &[u8]) -> Ordering {
    let mut left = left.iter().copied();
    let mut right = right.iter().copied();
    loop {
        match (left.next(), right.next()) {
            (None, None) => return Ordering::Equal,
            (None, Some(_)) => return Ordering::Less,
            (Some(_), None) => return Ordering::Greater,
            (Some(a), Some(b)) if a.to_ascii_lowercase() == b.to_ascii_lowercase() => continue,
            (Some(a), Some(b)) => return a.to_ascii_lowercase().cmp(&b.to_ascii_lowercase()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::casecmp;
    use core::cmp::Ordering;

    #[test]
    fn compares_symbols_without_regard_to_case() {
        assert_eq!(casecmp(b"abcdef", b"abcde"), Ordering::Greater);
        assert_eq!(casecmp(b"abcdef", b"abcde") as i32, 1);

        assert_eq!(casecmp(b"aBcDeF", b"abcdef"), Ordering::Equal);
        assert_eq!(casecmp(b"aBcDeF", b"abcdef") as i32, 0);

        assert_eq!(casecmp(b"abcdef", b"abcdefg"), Ordering::Less);
        assert_eq!(casecmp(b"abcdef", b"abcdefg") as i32, -1);

        assert_eq!(casecmp(b"abcdef", b"ABCDEF"), Ordering::Equal);
        assert_eq!(casecmp(b"abcdef", b"ABCDEF") as i32, 0);
    }

    #[test]
    fn doesent_consider_non_ascii_chars_equal_that_arent() {
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
        let cmp = casecmp(lower_a_tilde, lower_a_umlaut);
        assert!(!matches!(cmp, Ordering::Equal));
        let cmp = casecmp(lower_a_umlaut, lower_a_tilde);
        assert!(!matches!(cmp, Ordering::Equal));
        let cmp = casecmp(upper_a_tilde, upper_a_umlaut);
        assert!(!matches!(cmp, Ordering::Equal));
        let cmp = casecmp(upper_a_umlaut, upper_a_tilde);
        assert!(!matches!(cmp, Ordering::Equal));

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
        let cmp = casecmp(lower_a_tilde, lower_a_umlaut);
        assert!(!matches!(cmp, Ordering::Equal));
        let cmp = casecmp(lower_a_umlaut, lower_a_tilde);
        assert!(!matches!(cmp, Ordering::Equal));
        let cmp = casecmp(upper_a_tilde, upper_a_umlaut);
        assert!(!matches!(cmp, Ordering::Equal));
        let cmp = casecmp(upper_a_umlaut, upper_a_tilde);
        assert!(!matches!(cmp, Ordering::Equal));
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
        let cmp = casecmp(upper_a_tilde, lower_a_tilde);
        assert!(matches!(cmp, Ordering::Less));
        let cmp = casecmp(upper_a_umlaut, lower_a_umlaut);
        assert!(matches!(cmp, Ordering::Less));
        let cmp = casecmp(lower_a_tilde, upper_a_tilde);
        assert!(matches!(cmp, Ordering::Greater));
        let cmp = casecmp(lower_a_umlaut, upper_a_umlaut);
        assert!(matches!(cmp, Ordering::Greater));

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
        let cmp = casecmp(upper_a_tilde, lower_a_tilde);
        assert!(matches!(cmp, Ordering::Less));
        let cmp = casecmp(upper_a_umlaut, lower_a_umlaut);
        assert!(matches!(cmp, Ordering::Less));
        let cmp = casecmp(lower_a_tilde, upper_a_tilde);
        assert!(matches!(cmp, Ordering::Greater));
        let cmp = casecmp(lower_a_umlaut, upper_a_umlaut);
        assert!(matches!(cmp, Ordering::Greater));
    }

    #[test]
    fn exhaustive() {
        let lower = 'a'..'z';
        let upper = 'A'..'Z';
        let mut l_buf = [0; 4];
        let mut r_buf = [0; 4];
        for (left, right) in lower.zip(upper) {
            let left = left.encode_utf8(&mut l_buf);
            let right = right.encode_utf8(&mut r_buf);
            assert!(matches!(
                casecmp(left.as_bytes(), right.as_bytes()),
                Ordering::Equal
            ));
            assert!(matches!(
                casecmp(right.as_bytes(), left.as_bytes()),
                Ordering::Equal
            ));
        }
    }
}
