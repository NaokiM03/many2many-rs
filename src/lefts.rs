use core::fmt;

use hashbrown::{hash_map::Iter, HashSet};

use crate::{Many2Many, Ref};

pub struct Lefts<'a, Left, Right>(Iter<'a, Ref<Left>, HashSet<Ref<Right>>>);

impl<Left, Right> Clone for Lefts<'_, Left, Right> {
    fn clone(&self) -> Self {
        Lefts(self.0.clone())
    }
}

impl<Left, Right> fmt::Debug for Lefts<'_, Left, Right>
where
    Left: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

impl<'a, Left, Right> Iterator for Lefts<'a, Left, Right> {
    type Item = &'a Left;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(left, _right)| &(**left))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<Left, Right> ExactSizeIterator for Lefts<'_, Left, Right> {
    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<Left, Right> Many2Many<Left, Right> {
    pub fn lefts(&self) -> Lefts<'_, Left, Right> {
        Lefts(self.left.iter())
    }
}
