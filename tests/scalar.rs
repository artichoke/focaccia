use core::cell::Cell;
use core::cmp::Ordering;
use core::iter;

use focaccia::CaseFold;

fn scalar(hex: &str) -> char {
    char::from_u32(u32::from_str_radix(hex, 16).unwrap()).unwrap()
}

#[test]
fn scalar_comparisons_accept_every_bundled_fold_mapping() {
    for line in include_str!("../CaseFolding.txt").lines() {
        let data = line.split('#').next().unwrap().trim();
        if data.is_empty() {
            continue;
        }
        let mut fields = data.split(';').map(str::trim);
        let original = scalar(fields.next().unwrap());
        let status = fields.next().unwrap();
        let mapped = fields.next().unwrap();
        let strategies: &[CaseFold] = match status {
            "C" | "F" if original == 'I' || original == 'İ' => {
                &[CaseFold::Full, CaseFold::Lithuanian]
            }
            "C" | "F" => &[CaseFold::Full, CaseFold::Lithuanian, CaseFold::Turkic],
            "T" => &[CaseFold::Turkic],
            _ => continue,
        };
        for &strategy in strategies {
            let expected = || mapped.split_whitespace().map(scalar);
            assert_eq!(
                strategy.casecmp_chars(iter::once(original), expected()),
                Ordering::Equal,
                "{line}: {strategy:?}"
            );
            assert!(
                strategy.case_eq_chars(expected(), iter::once(original)),
                "{line}: {strategy:?}"
            );
        }
    }
}

#[test]
fn scalar_and_utf8_comparisons_have_the_same_ordering() {
    let inputs = [
        "",
        "a",
        "A",
        "ß",
        "ss",
        "İ",
        "I",
        "ı",
        "i\u{307}",
        "ﬃ",
        "ffi",
        "𐐀",
        "𐐨",
        "\0",
        "Σ",
        "ς",
        "São Paulo",
    ];
    for strategy in [
        CaseFold::Full,
        CaseFold::Ascii,
        CaseFold::Turkic,
        CaseFold::Lithuanian,
    ] {
        for left in inputs {
            for right in inputs {
                assert_eq!(
                    strategy.casecmp_chars(left.chars(), right.chars()),
                    strategy.casecmp(left, right)
                );
                assert_eq!(
                    strategy.case_eq_chars(left.chars(), right.chars()),
                    strategy.case_eq(left, right)
                );
            }
        }
    }
}

#[test]
fn callers_can_compare_utf16_including_supplementary_characters() {
    let upper = char::decode_utf16([0xD801, 0xDC00]).map(Result::unwrap);
    let lower = char::decode_utf16([0xD801, 0xDC28]).map(Result::unwrap);
    assert!(CaseFold::Full.case_eq_chars(upper, lower));
}

#[test]
fn comparisons_stop_at_the_first_folded_difference() {
    let left_reads = Cell::new(0);
    let right_reads = Cell::new(0);
    let left = ['a', 'ß', 'z']
        .into_iter()
        .inspect(|_| left_reads.set(left_reads.get() + 1));
    let right = ['b', 's', 's']
        .into_iter()
        .inspect(|_| right_reads.set(right_reads.get() + 1));
    assert_eq!(CaseFold::Full.casecmp_chars(left, right), Ordering::Less);
    assert_eq!(left_reads.get(), 1);
    assert_eq!(right_reads.get(), 1);
}

#[test]
fn ascii_pairs_preserve_ordering_in_every_strategy() {
    for strategy in [
        CaseFold::Full,
        CaseFold::Ascii,
        CaseFold::Turkic,
        CaseFold::Lithuanian,
    ] {
        for a in 0..=127_u8 {
            for b in 0..=127_u8 {
                let a = char::from(a);
                let b = char::from(b);
                let mut left = [0; 4];
                let mut right = [0; 4];
                let expected =
                    strategy.casecmp(a.encode_utf8(&mut left), b.encode_utf8(&mut right));
                assert_eq!(strategy.casecmp_chars([a], [b]), expected);
            }
        }
    }
}
