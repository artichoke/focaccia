use core::cmp::Ordering;

use super::mapping::Mode;

pub(super) enum Prefix<'a> {
    Complete(Ordering),
    Unicode(&'a str, &'a str),
}

/// Compare ASCII pairs without constructing Unicode mapping iterators. Stop
/// before either non-ASCII byte so both suffixes remain on character boundaries.
#[inline]
pub(super) fn compare<'a>(left: &'a str, right: &'a str, mode: Mode) -> Prefix<'a> {
    let mut index = 0;
    for (&left_byte, &right_byte) in left.as_bytes().iter().zip(right.as_bytes()) {
        if !left_byte.is_ascii() || !right_byte.is_ascii() {
            return Prefix::Unicode(&left[index..], &right[index..]);
        }
        let left_folded = fold(left_byte, mode);
        let right_folded = fold(right_byte, mode);
        match left_folded.cmp(&right_folded) {
            Ordering::Equal => index += 1,
            ordering => return Prefix::Complete(ordering),
        }
    }
    Prefix::Complete(left.len().cmp(&right.len()))
}

#[inline]
fn fold(byte: u8, mode: Mode) -> u32 {
    match (mode, byte) {
        (Mode::Turkic, b'I') => 0x0131,
        _ => u32::from(byte.to_ascii_lowercase()),
    }
}

#[cfg(test)]
mod tests {
    use core::cmp::Ordering;
    use std::string::ToString;

    use super::Mode;
    use crate::folding::mapping::lookup;
    use crate::{
        unicode_full_case_eq, unicode_full_casecmp, unicode_full_turkic_case_eq,
        unicode_full_turkic_casecmp,
    };

    fn check(left: &str, right: &str) {
        for mode in [Mode::Full, Mode::Turkic] {
            let expected = left
                .chars()
                .flat_map(|c| lookup(c, mode))
                .cmp(right.chars().flat_map(|c| lookup(c, mode)));
            let (actual, equal) = match mode {
                Mode::Full => (
                    unicode_full_casecmp(left, right),
                    unicode_full_case_eq(left, right),
                ),
                Mode::Turkic => (
                    unicode_full_turkic_casecmp(left, right),
                    unicode_full_turkic_case_eq(left, right),
                ),
            };
            assert_eq!(actual, expected, "{mode:?}: {left:?}, {right:?}");
            assert_eq!(
                equal,
                expected == Ordering::Equal,
                "{mode:?}: {left:?}, {right:?}"
            );
        }
    }

    #[test]
    fn every_ascii_pair_matches_unicode_folding() {
        for left in 0..=127_u8 {
            for right in 0..=127_u8 {
                let left = char::from(left).to_string();
                let right = char::from(right).to_string();
                check(&left, &right);
                // The fast prefix must resume folding at the same position on
                // both sides, including when a suffix expands to multiple chars.
                check(
                    &std::format!("Prefix:{left}ß"),
                    &std::format!("pREFIX:{right}SS"),
                );
            }
        }
    }

    #[test]
    fn unicode_suffixes_keep_character_boundaries_and_fold_expansions() {
        let corpus = [
            "",
            "a",
            "A",
            "I",
            "i",
            "ı",
            "İ",
            "ß",
            "ss",
            "ﬃ",
            "ffi",
            "\u{212a}",
            "k",
            "ΐ",
            "ι\u{308}\u{301}",
            "🦀",
            "🦁",
            "aß",
            "ASS",
            "aı",
            "AI",
            "aİ",
            "ai",
            "prefix:ﬃ",
            "PREFIX:FFI",
            "prefix:\0ß",
            "PREFIX:\0SS",
        ];
        for left in corpus {
            for right in corpus {
                check(left, right);
            }
        }
    }
}
