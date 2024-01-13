use core::fmt;

use std::{fmt::Debug, hash::Hash, rc::Rc};

use hashbrown::{hash_map::Iter, HashSet};

use crate::Many2Many;

pub struct Lefts<'a, Left, Right>(Iter<'a, Rc<Left>, HashSet<Rc<Right>>>)
where
    Left: Hash + Eq + Clone,
    Right: Hash + Eq + Clone;

impl<Left, Right> Clone for Lefts<'_, Left, Right>
where
    Left: Hash + Eq + Clone,
    Right: Hash + Eq + Clone,
{
    fn clone(&self) -> Self {
        Lefts(self.0.clone())
    }
}

impl<Left: Debug, Right> fmt::Debug for Lefts<'_, Left, Right>
where
    Left: Hash + Eq + Clone,
    Right: Hash + Eq + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

impl<'a, Left, Right> Iterator for Lefts<'a, Left, Right>
where
    Left: Hash + Eq + Clone,
    Right: Hash + Eq + Clone,
{
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

impl<Left, Right> ExactSizeIterator for Lefts<'_, Left, Right>
where
    Left: Hash + Eq + Clone,
    Right: Hash + Eq + Clone,
{
    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<Left, Right> Many2Many<Left, Right>
where
    Left: Hash + Eq + Clone,
    Right: Hash + Eq + Clone,
{
    pub fn lefts(&self) -> Lefts<'_, Left, Right> {
        Lefts(self.left.iter())
    }
}
