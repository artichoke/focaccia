//! Independent checks against the data file, without the lookup-table generator.

use core::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::format;
use std::string::String;
use std::vec::Vec;

use super::{lookup, Mode};
use crate::{
    ascii_case_eq, ascii_casecmp, unicode_full_case_eq, unicode_full_casecmp,
    unicode_full_lithuanian_case_eq, unicode_full_lithuanian_casecmp, unicode_full_turkic_case_eq,
    unicode_full_turkic_casecmp, CaseFold,
};

struct Oracle {
    full: BTreeMap<char, String>,
    turkic: BTreeMap<char, String>,
}

impl Oracle {
    fn new() -> Self {
        let mut full = BTreeMap::new();
        let mut overrides = BTreeMap::new();
        for line in include_str!("../../../CaseFolding.txt").lines() {
            let data = line.split('#').next().unwrap().trim();
            if data.is_empty() {
                continue;
            }
            let mut fields = data.split(';').map(str::trim);
            let code =
                char::from_u32(u32::from_str_radix(fields.next().unwrap(), 16).unwrap()).unwrap();
            let status = fields.next().unwrap();
            let mapping: String = fields
                .next()
                .unwrap()
                .split_whitespace()
                .map(|hex| char::from_u32(u32::from_str_radix(hex, 16).unwrap()).unwrap())
                .collect();
            match status {
                "C" | "F" => assert!(full.insert(code, mapping).is_none()),
                "T" => assert!(overrides.insert(code, mapping).is_none()),
                "S" => {} // Simple mappings do not participate in full folding.
                _ => panic!("Unknown case folding status: {status}"),
            }
        }
        assert!(!full.is_empty());
        assert_eq!(overrides.len(), 2);
        let mut turkic = full.clone();
        turkic.extend(overrides);
        Self { full, turkic }
    }

    fn fold(&self, text: &str, scheme: CaseFold) -> String {
        let mappings = match scheme {
            CaseFold::Turkic => &self.turkic,
            _ => &self.full,
        };
        let mut output = String::new();
        for ch in text.chars() {
            if scheme == CaseFold::Ascii {
                output.push(match ch {
                    'A'..='Z' => char::from_u32(u32::from(ch) + 32).unwrap(),
                    _ => ch,
                });
            } else if let Some(mapped) = mappings.get(&ch) {
                output.push_str(mapped);
            } else {
                output.push(ch);
            }
        }
        output
    }
}

const SCHEMES: [CaseFold; 4] = [
    CaseFold::Full,
    CaseFold::Turkic,
    CaseFold::Lithuanian,
    CaseFold::Ascii,
];

fn check(scheme: CaseFold, left: &str, right: &str, expected: Ordering) {
    assert_eq!(
        scheme.casecmp(left, right),
        expected,
        "{scheme:?}: {left:?}, {right:?}"
    );
    assert_eq!(
        scheme.case_eq(left, right),
        expected.is_eq(),
        "{scheme:?}: {left:?}, {right:?}"
    );
    let (order, equal) = match scheme {
        CaseFold::Full => (
            unicode_full_casecmp(left, right),
            unicode_full_case_eq(left, right),
        ),
        CaseFold::Turkic => (
            unicode_full_turkic_casecmp(left, right),
            unicode_full_turkic_case_eq(left, right),
        ),
        CaseFold::Lithuanian => (
            unicode_full_lithuanian_casecmp(left, right),
            unicode_full_lithuanian_case_eq(left, right),
        ),
        CaseFold::Ascii => (
            ascii_casecmp(left.as_bytes(), right.as_bytes()),
            ascii_case_eq(left.as_bytes(), right.as_bytes()),
        ),
    };
    assert_eq!(order, expected, "{scheme:?}: {left:?}, {right:?}");
    assert_eq!(equal, expected.is_eq(), "{scheme:?}: {left:?}, {right:?}");
}

#[test]
fn every_scalar_has_exact_mapping_and_iterator_contract() {
    let oracle = Oracle::new();
    let mut count = 0;
    for ch in char::MIN..=char::MAX {
        let mut buf = [0; 4];
        let input = ch.encode_utf8(&mut buf);
        for (mode, scheme) in [
            (Mode::Full, CaseFold::Full),
            (Mode::Turkic, CaseFold::Turkic),
        ] {
            let expected = oracle.fold(input, scheme);
            let mapping = lookup(ch, mode);
            let mut remaining = expected.chars().count();
            assert_eq!(mapping.clone().into_iter().count(), remaining);
            let mut iter = mapping.into_iter();
            for output in expected.chars() {
                assert_eq!(iter.len(), remaining);
                assert_eq!(iter.size_hint(), (remaining, Some(remaining)));
                assert_eq!(
                    iter.next(),
                    Some(u32::from(output)),
                    "{mode:?}: U+{:04X}",
                    u32::from(ch)
                );
                remaining -= 1;
            }
            assert_eq!(iter.len(), 0);
            assert_eq!(iter.size_hint(), (0, Some(0)));
            assert_eq!(iter.next(), None);
            assert_eq!(iter.next(), None);
        }
        for scheme in SCHEMES {
            let expected = oracle.fold(input, scheme);
            check(scheme, input, &expected, Ordering::Equal);
            check(scheme, input, "", Ordering::Greater);
        }
        count += 1;
    }
    assert_eq!(count, 1_112_064);
}

#[test]
fn every_byte_pair_and_two_byte_sequence() {
    let fold = |byte: u8| match byte {
        b'A'..=b'Z' => byte + 32,
        _ => byte,
    };
    for left in u8::MIN..=u8::MAX {
        for right in u8::MIN..=u8::MAX {
            let expected = fold(left).cmp(&fold(right));
            assert_eq!(ascii_casecmp(&[left], &[right]), expected);
            assert_eq!(ascii_case_eq(&[left], &[right]), expected.is_eq());
            let input = [left, right];
            let folded = [fold(left), fold(right)];
            assert_eq!(ascii_casecmp(&input, &folded), Ordering::Equal);
            assert!(ascii_case_eq(&input, &folded));
            assert_eq!(ascii_casecmp(&input, &folded[..1]), Ordering::Greater);
            assert_eq!(ascii_casecmp(&folded[..1], &input), Ordering::Less);
            assert!(!ascii_case_eq(&input, &folded[..1]));
        }
    }
}

fn check_corpus(oracle: &Oracle, corpus: &BTreeSet<String>) {
    for scheme in SCHEMES {
        let folded: Vec<_> = corpus.iter().map(|s| (s, oracle.fold(s, scheme))).collect();
        for (left, left_folded) in &folded {
            for (right, right_folded) in &folded {
                check(scheme, left, right, left_folded.cmp(right_folded));
            }
        }
    }
}

#[test]
#[ignore = "large pairwise audit; run cargo test --release -- --ignored"]
fn every_pair_of_mapping_sources_and_outputs() {
    let oracle = Oracle::new();
    let mut corpus = BTreeSet::from([
        String::new(),
        String::from("\0"),
        String::from("\u{10FFFF}"),
    ]);
    for (ch, mapping) in oracle.full.iter().chain(&oracle.turkic) {
        corpus.insert(String::from(*ch));
        corpus.insert(mapping.clone());
    }
    std::println!(
        "{} strings, {} ordered pairs per strategy",
        corpus.len(),
        corpus.len() * corpus.len()
    );
    check_corpus(&oracle, &corpus);
}

#[test]
fn expansion_boundaries_and_combining_marks() {
    let oracle = Oracle::new();
    let mut corpus = BTreeSet::new();
    for (ch, mapping) in &oracle.full {
        if mapping.chars().count() < 2 {
            continue;
        }
        for suffix in ["", "\0", "A", "\u{301}", "\u{307}", "\u{10FFFF}"] {
            corpus.insert(format!("{ch}{suffix}"));
            corpus.insert(format!("{mapping}{suffix}"));
        }
        // Put each expansion on opposite sides of an input character boundary.
        for other in oracle
            .full
            .keys()
            .filter(|c| oracle.full[c].chars().count() > 1)
        {
            let input = format!("{ch}{other}");
            for scheme in SCHEMES {
                let folded = oracle.fold(&input, scheme);
                check(scheme, &input, &folded, Ordering::Equal);
                check(scheme, &folded, &input, Ordering::Equal);
            }
        }
    }
    check_corpus(&oracle, &corpus);
}
