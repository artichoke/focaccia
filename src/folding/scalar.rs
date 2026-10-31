//! Case folding comparisons over caller-decoded Unicode scalars.

use core::cmp::Ordering;
use core::iter;

use super::mapping::{self, Mapping, Mode};
use crate::CaseFold;

fn mapping(strategy: CaseFold, character: char) -> Mapping {
    match strategy {
        CaseFold::Ascii => Mapping::Single(character.to_ascii_lowercase() as u32),
        CaseFold::Full | CaseFold::Lithuanian => mapping::lookup(character, Mode::Full),
        CaseFold::Turkic => mapping::lookup(character, Mode::Turkic),
    }
}

pub(crate) fn casecmp<L, R>(strategy: CaseFold, left: L, right: R) -> Ordering
where
    L: IntoIterator<Item = char>,
    R: IntoIterator<Item = char>,
{
    let mut left = left.into_iter();
    let mut right = right.into_iter();
    loop {
        match (left.next(), right.next()) {
            (Some(a), Some(b)) if a.is_ascii() && b.is_ascii() => {
                let fold = |character: char| match (strategy, character) {
                    (CaseFold::Turkic, 'I') => 'ı',
                    _ => character.to_ascii_lowercase(),
                };
                match fold(a).cmp(&fold(b)) {
                    Ordering::Equal => {}
                    ordering => return ordering,
                }
            }
            (Some(a), Some(b)) => {
                return iter::once(a)
                    .chain(left)
                    .flat_map(|character| mapping(strategy, character))
                    .cmp(
                        iter::once(b)
                            .chain(right)
                            .flat_map(|character| mapping(strategy, character)),
                    );
            }
            (None, None) => return Ordering::Equal,
            (None, Some(_)) => return Ordering::Less,
            (Some(_), None) => return Ordering::Greater,
        }
    }
}
