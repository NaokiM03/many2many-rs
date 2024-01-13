use core::fmt;
use std::{
    collections::{hash_map::Keys, HashSet},
    fmt::Debug,
    hash::Hash,
    rc::Rc,
};

pub struct Rights<'a, Left, Right>(Keys<'a, Rc<Right>, HashSet<Rc<Left>>>)
where
    Left: Hash + Eq + Clone,
    Right: Hash + Eq + Clone;

impl<Left, Right> Rights<'_, Left, Right>
where
    Left: Hash + Eq + Clone,
    Right: Hash + Eq + Clone,
{
    pub fn new(inner: Keys<'_, Rc<Right>, HashSet<Rc<Left>>>) -> Rights<'_, Left, Right> {
        Rights(inner)
    }
}

impl<Left, Right> Clone for Rights<'_, Left, Right>
where
    Left: Hash + Eq + Clone,
    Right: Hash + Eq + Clone,
{
    fn clone(&self) -> Self {
        Rights(self.0.clone())
    }
}

impl<Left, Right: Debug> fmt::Debug for Rights<'_, Left, Right>
where
    Left: Hash + Eq + Clone,
    Right: Hash + Eq + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.0.clone()).finish()
    }
}

impl<'a, Left, Right> Iterator for Rights<'a, Left, Right>
where
    Left: Hash + Eq + Clone,
    Right: Hash + Eq + Clone,
{
    type Item = &'a Right;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|x| &(**x))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<Left, Right> ExactSizeIterator for Rights<'_, Left, Right>
where
    Left: Hash + Eq + Clone,
    Right: Hash + Eq + Clone,
{
    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }
}
