use core::fmt;

use hashbrown::{hash_map::IntoIter, HashSet};

use crate::{Many2Many, Ref};

pub struct IntoRights<Left, Right>(IntoIter<Ref<Right>, HashSet<Ref<Left>>>);

impl<Left, Right> fmt::Debug for IntoRights<Left, Right>
where
    Right: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list()
            .entries(self.0.rustc_iter().map(|(right, _left)| right))
            .finish()
    }
}

impl<Left, Right> Iterator for IntoRights<Left, Right> {
    type Item = Right;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .next()
            .map(|(right, _left)| Ref::try_unwrap(right).ok().unwrap())
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<Left, Right> ExactSizeIterator for IntoRights<Left, Right> {
    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<Left, Right> Many2Many<Left, Right> {
    pub fn into_rights(self) -> IntoRights<Left, Right> {
        IntoRights(self.right.into_iter())
    }
}
