use core::fmt;

use hashbrown::{hash_map::Iter, HashSet};

use crate::{Many2Many, Ref};

pub struct Rights<'a, Left, Right>(Iter<'a, Ref<Right>, HashSet<Ref<Left>>>);

impl<Left, Right> Clone for Rights<'_, Left, Right> {
    fn clone(&self) -> Self {
        Rights(self.0.clone())
    }
}

impl<Left, Right> fmt::Debug for Rights<'_, Left, Right>
where
    Right: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

impl<'a, Left, Right> Iterator for Rights<'a, Left, Right> {
    type Item = &'a Right;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(right, _left)| &(**right))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<Left, Right> ExactSizeIterator for Rights<'_, Left, Right> {
    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<Left, Right> Many2Many<Left, Right> {
    pub fn rights(&self) -> Rights<'_, Left, Right> {
        Rights(self.right.iter())
    }
}
