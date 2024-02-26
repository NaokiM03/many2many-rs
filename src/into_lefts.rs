use core::fmt::{self, Debug};

use std::hash::Hash;

use hashbrown::{hash_map::IntoIter, HashSet};

use crate::{Many2Many, Ref};

pub struct IntoLefts<Left, Right>(IntoIter<Ref<Left>, HashSet<Ref<Right>>>)
where
    Left: Hash + Eq + Clone,
    Right: Hash + Eq + Clone;

impl<Left, Right> Iterator for IntoLefts<Left, Right>
where
    Left: Hash + Eq + Clone,
    Right: Hash + Eq + Clone,
{
    type Item = Left;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .next()
            .map(|(left, _right)| Ref::try_unwrap(left).unwrap_or_else(|rc| (*rc).clone()))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<Left, Right> ExactSizeIterator for IntoLefts<Left, Right>
where
    Left: Hash + Eq + Clone,
    Right: Hash + Eq + Clone,
{
    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<Left: Debug, Right> Debug for IntoLefts<Left, Right>
where
    Left: Hash + Eq + Clone,
    Right: Hash + Eq + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list()
            .entries(self.0.rustc_iter().map(|(left, _right)| left))
            .finish()
    }
}

impl<Left, Right> Many2Many<Left, Right>
where
    Left: Hash + Eq + Clone,
    Right: Hash + Eq + Clone,
{
    pub fn into_lefts(self) -> IntoLefts<Left, Right> {
        IntoLefts(self.left.into_iter())
    }
}
