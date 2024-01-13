use core::fmt;
use std::{
    collections::{hash_map::Keys, HashSet},
    fmt::Debug,
    hash::Hash,
    rc::Rc,
};

pub struct Lefts<'a, Left, Right>(Keys<'a, Rc<Left>, HashSet<Rc<Right>>>)
where
    Left: Hash + Eq + Clone,
    Right: Hash + Eq + Clone;

impl<Left, Right> Lefts<'_, Left, Right>
where
    Left: Hash + Eq + Clone,
    Right: Hash + Eq + Clone,
{
    pub fn new(inner: Keys<'_, Rc<Left>, HashSet<Rc<Right>>>) -> Lefts<'_, Left, Right> {
        Lefts(inner)
    }
}

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
        f.debug_list().entries(self.0.clone()).finish()
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
        self.0.next().map(|x| &(**x))
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
