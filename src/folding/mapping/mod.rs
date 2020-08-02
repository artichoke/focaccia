use core::iter::FusedIterator;

mod lookup;

pub use lookup::lookup;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Mode {
    Full,
    Turkic,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Mapping {
    Single(u32),
    Double(u32, u32),
    Triple(u32, u32, u32),
}

impl IntoIterator for Mapping {
    type Item = u32;
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
    type Item = u32;

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
