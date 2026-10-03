mod support;

use std::hint::black_box;

#[derive(Clone, Copy)]
enum Mode {
    Ascii,
    Full,
    Turkic,
}

fn main() {
    support::header();
    let ascii_left = "The Quick Brown FOX jumps over the lazy DOG! 0123456789\n".repeat(64);
    let ascii_right = ascii_left.to_ascii_lowercase();
    let unicode_left = "Straße Αύριο 東京 🦀\n".repeat(32);
    let unicode_right = "STRASSE αύριο 東京 🦀\n".repeat(32);
    let mut late_mismatch = ascii_right.clone();
    late_mismatch.push('x');
    let inputs = [
        ("empty", String::new(), String::new()),
        (
            "ruby_name",
            "Artichoke::String#casecmp?".into(),
            "artichoke::string#casecmp?".into(),
        ),
        (
            "early_mismatch",
            format!("a{ascii_left}"),
            format!("b{ascii_right}"),
        ),
        ("late_mismatch", ascii_left.clone(), late_mismatch),
        ("ascii_long", ascii_left, ascii_right),
        ("mixed", unicode_left, unicode_right),
        (
            "expansions",
            "ßﬃΐᾀ".repeat(64),
            "ssffiι\u{308}\u{301}ἀι".repeat(64),
        ),
        (
            "turkic",
            "Iİ Istanbul İSTANBUL\n".repeat(32),
            "ıi ıstanbul istanbul\n".repeat(32),
        ),
        (
            "unicode_early_mismatch",
            "🦀Straße".into(),
            "🦁STRASSE".into(),
        ),
        (
            "unicode_after_ascii",
            "prefix: \u{212a}elvin".into(),
            "PREFIX: kelvin".into(),
        ),
    ];
    for (input_name, left, right) in &inputs {
        for (mode_name, mode) in [
            ("ascii", Mode::Ascii),
            ("full", Mode::Full),
            ("turkic", Mode::Turkic),
        ] {
            support::measure(
                &format!("{mode_name}/{input_name}/casecmp"),
                left.len() + right.len(),
                || {
                    let (left, right) = (black_box(left.as_str()), black_box(right.as_str()));
                    black_box(match mode {
                        Mode::Ascii => focaccia::ascii_casecmp(left.as_bytes(), right.as_bytes()),
                        Mode::Full => focaccia::unicode_full_casecmp(left, right),
                        Mode::Turkic => focaccia::unicode_full_turkic_casecmp(left, right),
                    });
                },
            );
            support::measure(
                &format!("{mode_name}/{input_name}/case_eq"),
                left.len() + right.len(),
                || {
                    let (left, right) = (black_box(left.as_str()), black_box(right.as_str()));
                    black_box(match mode {
                        Mode::Ascii => focaccia::ascii_case_eq(left.as_bytes(), right.as_bytes()),
                        Mode::Full => focaccia::unicode_full_case_eq(left, right),
                        Mode::Turkic => focaccia::unicode_full_turkic_case_eq(left, right),
                    });
                },
            );
        }
    }
}
