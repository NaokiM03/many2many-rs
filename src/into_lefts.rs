use core::fmt;

use hashbrown::{hash_map::IntoIter, HashSet};

use crate::{Many2Many, Ref};

pub struct IntoLefts<Left, Right>(IntoIter<Ref<Left>, HashSet<Ref<Right>>>);

impl<Left, Right> fmt::Debug for IntoLefts<Left, Right>
where
    Left: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list()
            .entries(self.0.rustc_iter().map(|(left, _right)| left))
            .finish()
    }
}

impl<Left, Right> Iterator for IntoLefts<Left, Right> {
    type Item = Left;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .next()
            .map(|(left, _right)| Ref::try_unwrap(left).ok().unwrap())
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<Left, Right> ExactSizeIterator for IntoLefts<Left, Right> {
    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<Left, Right> Many2Many<Left, Right> {
    pub fn into_lefts(self) -> IntoLefts<Left, Right> {
        IntoLefts(self.left.into_iter())
    }
}

#[cfg(test)]
mod tests {
    use crate::Many2Many;

    #[test]
    fn test_into_lefts() {
        let mut map = Many2Many::new();
        assert!(map.insert(1, "a"));
        assert!(map.insert(1, "b"));
        assert!(map.insert(2, "a"));
        assert!(map.insert(2, "b"));

        let lefts = map.into_lefts();
        assert_eq!(lefts.len(), 2);

        let mut lefts: Vec<i32> = lefts.collect();
        lefts.sort();
        assert_eq!(lefts[0], 1);
        assert_eq!(lefts[1], 2);
    }
}
