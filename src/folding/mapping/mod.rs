use core::iter::FusedIterator;

mod lookup;

pub use lookup::lookup;

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Full,
    Turkic,
}

#[derive(Debug, Clone)]
pub enum Mapping {
    Empty,
    Single(u32),
    Double(u32, u32),
    Triple(u32, u32, u32),
}

impl Mapping {
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        match self {
            Self::Empty => 0,
            Self::Single(..) => 1,
            Self::Double(..) => 2,
            Self::Triple(..) => 3,
        }
    }
}

impl IntoIterator for Mapping {
    type Item = u32;
    type IntoIter = Iter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Iter(self)
    }
}

#[derive(Debug)]
pub struct Iter(Mapping);

impl Iterator for Iter {
    type Item = u32;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let next = match self.0 {
            Mapping::Empty => return None,
            Mapping::Single(ch) => {
                self.0 = Mapping::Empty;
                ch
            }
            Mapping::Double(ch, next) => {
                self.0 = Mapping::Single(next);
                ch
            }
            Mapping::Triple(ch, next, after) => {
                self.0 = Mapping::Double(next, after);
                ch
            }
        };
        Some(next)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.0.len();
        (len, Some(len))
    }

    #[inline]
    fn count(self) -> usize {
        self.0.len()
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

        let iter = mapping.clone().into_iter();
        assert_eq!(iter.size_hint(), (1_usize, Some(1_usize)));
        assert_eq!(iter.count(), 1);

        let mut iter = mapping.clone().into_iter();
        iter.next();
        assert_eq!(iter.size_hint(), (0_usize, Some(0_usize)));
        assert_eq!(iter.count(), 0);

        let mut iter = mapping.clone().into_iter();
        iter.next();
        iter.next();
        assert_eq!(iter.size_hint(), (0_usize, Some(0_usize)));
        assert_eq!(iter.count(), 0);

        let mut iter = mapping.clone().into_iter();
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

        let iter = mapping.clone().into_iter();
        assert_eq!(iter.size_hint(), (2_usize, Some(2_usize)));
        assert_eq!(iter.count(), 2);

        let mut iter = mapping.clone().into_iter();
        iter.next();
        assert_eq!(iter.size_hint(), (1_usize, Some(1_usize)));
        assert_eq!(iter.count(), 1);

        let mut iter = mapping.clone().into_iter();
        iter.next();
        iter.next();
        assert_eq!(iter.size_hint(), (0_usize, Some(0_usize)));
        assert_eq!(iter.count(), 0);

        let mut iter = mapping.clone().into_iter();
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

        let iter = mapping.clone().into_iter();
        assert_eq!(iter.size_hint(), (3_usize, Some(3_usize)));
        assert_eq!(iter.count(), 3);

        let mut iter = mapping.clone().into_iter();
        iter.next();
        assert_eq!(iter.size_hint(), (2_usize, Some(2_usize)));
        assert_eq!(iter.count(), 2);

        let mut iter = mapping.clone().into_iter();
        iter.next();
        iter.next();
        assert_eq!(iter.size_hint(), (1_usize, Some(1_usize)));
        assert_eq!(iter.count(), 1);

        let mut iter = mapping.clone().into_iter();
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
