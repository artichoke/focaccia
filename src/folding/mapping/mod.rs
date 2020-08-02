use core::iter::FusedIterator;

pub mod full;
pub mod turkic;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Mapping {
    Single(char),
    Double(char, char),
    Triple(char, char, char),
}

impl IntoIterator for Mapping {
    type Item = char;
    type IntoIter = Iter;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            mapping: self,
            index: 0,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Iter {
    mapping: Mapping,
    index: u8,
}

impl Iterator for Iter {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let next = match self.mapping {
            Mapping::Single(ch) if self.index == 0 => ch,
            Mapping::Double(ch, _) if self.index == 0 => ch,
            Mapping::Double(_, ch) if self.index == 1 => ch,
            Mapping::Triple(ch, _, _) if self.index == 0 => ch,
            Mapping::Triple(_, ch, _) if self.index == 1 => ch,
            Mapping::Triple(_, _, ch) if self.index == 2 => ch,
            _ => return None,
        };
        self.index += 1;
        Some(next)
    }

    fn count(self) -> usize {
        let remaining = match self.mapping {
            Mapping::Single(_) => 1_u8.checked_sub(self.index),
            Mapping::Double(_, _) => 2_u8.checked_sub(self.index),
            Mapping::Triple(_, _, _) => 3_u8.checked_sub(self.index),
        };
        remaining.unwrap_or_default().into()
    }
}

impl ExactSizeIterator for Iter {}

impl FusedIterator for Iter {}