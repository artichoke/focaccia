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

    #[inline]
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

    #[inline]
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

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = match self.mapping {
            Mapping::Single(_) => 1_u8.checked_sub(self.index),
            Mapping::Double(_, _) => 2_u8.checked_sub(self.index),
            Mapping::Triple(_, _, _) => 3_u8.checked_sub(self.index),
        };
        let remaining = remaining.unwrap_or_default();
        (remaining.into(), Some(remaining.into()))
    }

    #[inline]
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

#[cfg(test)]
mod tests {
    use super::Mapping;

    #[test]
    fn mapping_single_iter() {
        let mapping = Mapping::Single(20);
        let mut iter = mapping.into_iter();
        assert_eq!(iter.next(), Some(20_u32));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn mapping_single_count() {
        let mapping = Mapping::Single(20);
        let iter = mapping.into_iter();
        assert_eq!(iter.size_hint(), (1_usize, Some(1_usize)));
        assert_eq!(iter.count(), 1);
        let mut iter = mapping.into_iter();
        iter.next();
        assert_eq!(iter.size_hint(), (0_usize, Some(0_usize)));
        assert_eq!(iter.count(), 0);
        let mut iter = mapping.into_iter();
        iter.next();
        iter.next();
        assert_eq!(iter.size_hint(), (0_usize, Some(0_usize)));
        assert_eq!(iter.count(), 0);
        let mut iter = mapping.into_iter();
        iter.next();
        iter.next();
        iter.next();
        assert_eq!(iter.size_hint(), (0_usize, Some(0_usize)));
        assert_eq!(iter.count(), 0);
        let mut iter = mapping.into_iter();
        iter.next();
        iter.next();
        iter.next();
        iter.next();
        assert_eq!(iter.size_hint(), (0_usize, Some(0_usize)));
        assert_eq!(iter.count(), 0);
    }

    #[test]
    fn mapping_double_iter() {
        let mapping = Mapping::Double(20, 30);
        let mut iter = mapping.into_iter();
        assert_eq!(iter.next(), Some(20_u32));
        assert_eq!(iter.next(), Some(30_u32));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn mapping_double_count() {
        let mapping = Mapping::Double(20, 30);
        let iter = mapping.into_iter();
        assert_eq!(iter.size_hint(), (2_usize, Some(2_usize)));
        assert_eq!(iter.count(), 2);
        let mut iter = mapping.into_iter();
        iter.next();
        assert_eq!(iter.size_hint(), (1_usize, Some(1_usize)));
        assert_eq!(iter.count(), 1);
        let mut iter = mapping.into_iter();
        iter.next();
        iter.next();
        assert_eq!(iter.size_hint(), (0_usize, Some(0_usize)));
        assert_eq!(iter.count(), 0);
        let mut iter = mapping.into_iter();
        iter.next();
        iter.next();
        iter.next();
        assert_eq!(iter.size_hint(), (0_usize, Some(0_usize)));
        assert_eq!(iter.count(), 0);
        let mut iter = mapping.into_iter();
        iter.next();
        iter.next();
        iter.next();
        iter.next();
        assert_eq!(iter.size_hint(), (0_usize, Some(0_usize)));
        assert_eq!(iter.count(), 0);
    }

    #[test]
    fn mapping_triple_iter() {
        let mapping = Mapping::Triple(20, 30, 40);
        let mut iter = mapping.into_iter();
        assert_eq!(iter.next(), Some(20_u32));
        assert_eq!(iter.next(), Some(30_u32));
        assert_eq!(iter.next(), Some(40_u32));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn mapping_triple_count() {
        let mapping = Mapping::Triple(20, 30, 40);
        let iter = mapping.into_iter();
        assert_eq!(iter.size_hint(), (3_usize, Some(3_usize)));
        assert_eq!(iter.count(), 3);
        let mut iter = mapping.into_iter();
        iter.next();
        assert_eq!(iter.size_hint(), (2_usize, Some(2_usize)));
        assert_eq!(iter.count(), 2);
        let mut iter = mapping.into_iter();
        iter.next();
        iter.next();
        assert_eq!(iter.size_hint(), (1_usize, Some(1_usize)));
        assert_eq!(iter.count(), 1);
        let mut iter = mapping.into_iter();
        iter.next();
        iter.next();
        iter.next();
        assert_eq!(iter.size_hint(), (0_usize, Some(0_usize)));
        assert_eq!(iter.count(), 0);
        let mut iter = mapping.into_iter();
        iter.next();
        iter.next();
        iter.next();
        iter.next();
        assert_eq!(iter.size_hint(), (0_usize, Some(0_usize)));
        assert_eq!(iter.count(), 0);
    }
}
