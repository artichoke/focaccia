use core::cmp::Ordering;

use focaccia::{unicode_full_lithuanian_case_eq, unicode_full_lithuanian_casecmp, CaseFold};

#[test]
fn ruby_compatible_lithuanian_comparisons() {
    // Ruby's :lithuanian option does not implement Lithuanian special casing.
    // Comparison still uses full folding (Ruby's downcase(:fold)), not downcasing.
    // Explicit expected orderings also guard against normalization and dot removal.
    let cases = [
        ("", "", Ordering::Equal),
        ("", "I", Ordering::Less),
        ("I", "i", Ordering::Equal),
        ("I", "ı", Ordering::Less),
        ("İ", "i\u{307}", Ordering::Equal),
        ("İ", "i", Ordering::Greater),
        ("I\u{300}", "i\u{300}", Ordering::Equal),
        ("I\u{301}", "i\u{301}", Ordering::Equal),
        ("J\u{301}", "j\u{301}", Ordering::Equal),
        ("Į\u{301}", "į\u{301}", Ordering::Equal),
        ("I\u{301}", "i\u{307}\u{301}", Ordering::Less),
        ("J\u{301}", "j\u{307}\u{301}", Ordering::Less),
        ("Į\u{301}", "į\u{307}\u{301}", Ordering::Less),
        ("I\u{323}\u{301}", "i\u{323}\u{301}", Ordering::Equal),
        ("I\u{307}\u{301}", "i\u{301}", Ordering::Greater),
        ("J\u{307}", "j", Ordering::Greater),
        ("Ì", "ì", Ordering::Equal),
        ("Í", "í", Ordering::Equal),
        ("Ĩ", "ĩ", Ordering::Equal),
        ("Ì", "i\u{307}\u{300}", Ordering::Greater),
        ("Í", "i\u{307}\u{301}", Ordering::Greater),
        ("Ĩ", "i\u{307}\u{303}", Ordering::Greater),
        ("Í", "i\u{301}", Ordering::Greater),
        ("ĄČĘĖĮŠŲŪŽ", "ąčęėįšųūž", Ordering::Equal),
        ("MASSE", "Maße", Ordering::Equal),
        ("ﬃ", "FFI", Ordering::Equal),
        ("Σς", "σσ", Ordering::Equal),
        ("I\0J", "i\0j", Ordering::Equal),
        ("I\u{301}", "i\u{301}j", Ordering::Less),
    ];

    for (left, right, expected) in cases {
        for (left, right, expected) in [(left, right, expected), (right, left, expected.reverse())]
        {
            assert_eq!(
                unicode_full_lithuanian_casecmp(left, right),
                expected,
                "{left:?}, {right:?}"
            );
            assert_eq!(
                unicode_full_lithuanian_case_eq(left, right),
                expected == Ordering::Equal,
                "{left:?}, {right:?}"
            );
            assert_eq!(CaseFold::Lithuanian.casecmp(left, right), expected);
            assert_eq!(
                CaseFold::Lithuanian.case_eq(left, right),
                expected == Ordering::Equal
            );
        }
    }
}

#[test]
fn parse_lithuanian_strategy() {
    assert_eq!(
        CaseFold::try_from(Some("lithuanian")),
        Ok(CaseFold::Lithuanian)
    );
    assert_eq!(
        CaseFold::try_from(Some(b"lithuanian".as_slice())),
        Ok(CaseFold::Lithuanian)
    );
}
