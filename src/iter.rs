use core::fmt;
use core::iter::FusedIterator;

use hashbrown::{hash_map::Iter as HashMapIter, hash_set::Iter as HashSetIter, HashSet};

use crate::{Many2Many, Ref};

pub struct Iter<'a, Left, Right> {
    current: Option<(&'a Ref<Left>, HashSetIter<'a, Ref<Right>>)>,
    rest: HashMapIter<'a, Ref<Left>, HashSet<Ref<Right>>>,
}

impl<Left, Right> Clone for Iter<'_, Left, Right> {
    fn clone(&self) -> Self {
        Iter {
            current: self.current.clone(),
            rest: self.rest.clone(),
        }
    }
}

impl<Left, Right> fmt::Debug for Iter<'_, Left, Right>
where
    Left: fmt::Debug,
    Right: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.clone()).finish()
    }
}

impl<'a, Left, Right> Iterator for Iter<'a, Left, Right> {
    type Item = (&'a Left, &'a Right);

    fn next(&mut self) -> Option<Self::Item> {
        let Some((left, rights)) = &mut self.current else {
            return None;
        };

        if let Some(right) = rights.next() {
            return Some((left, right));
        }

        self.current = self.rest.next().map(|(left, rights)| (left, rights.iter()));

        let Some((left, rights)) = &mut self.current else {
            return None;
        };

        if let Some(right) = rights.next() {
            return Some((left, right));
        }

        None
    }
}

impl<Left, Right> ExactSizeIterator for Iter<'_, Left, Right> {
    fn len(&self) -> usize {
        let mut len = 0;

        if let Some((_, rights)) = &self.current {
            len += rights.len();
        }

        for (_, rights) in self.rest.clone() {
            len += rights.len();
        }

        len
    }
}

impl<Left, Right> FusedIterator for Iter<'_, Left, Right> {}

impl<Left, Right> Many2Many<Left, Right> {
    pub fn iter(&self) -> Iter<'_, Left, Right> {
        let mut rest = self.left.iter();
        let current = rest.next().map(|(left, rights)| (left, rights.iter()));
        Iter { current, rest }
    }
}

#[cfg(test)]
mod tests {
    use crate::Many2Many;

    #[test]
    fn test_iter() {
        let mut map = Many2Many::from([(1, "a"), (1, "b")]);
        let mut iter = map.iter();
        let cloned = iter.clone();

        assert!(iter.next().is_some());
        assert!(iter.next().is_some());

        assert!(iter.next().is_none());
        assert_eq!(iter.len(), 0);

        assert_eq!(cloned.len(), 2);

        map.insert(1, "c");
    }
}
