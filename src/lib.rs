use std::{fmt::Debug, hash::Hash};

use hashbrown::{HashMap, HashSet};

mod into_lefts;
mod into_rights;
mod iter;
mod lefts;
mod rights;

#[cfg(feature = "thread_safe")]
pub type Ref<T> = std::sync::Arc<T>;
#[cfg(not(feature = "thread_safe"))]
pub type Ref<T> = std::rc::Rc<T>;

#[derive(Debug)]
pub struct Many2Many<Left, Right> {
    left: HashMap<Ref<Left>, HashSet<Ref<Right>>>,
    right: HashMap<Ref<Right>, HashSet<Ref<Left>>>,
}

impl<Left, Right> Clone for Many2Many<Left, Right> {
    #[inline]
    fn clone(&self) -> Self {
        Many2Many {
            left: self.left.clone(),
            right: self.right.clone(),
        }
    }
}

impl<Left, Right> PartialEq for Many2Many<Left, Right>
where
    Left: Hash + Eq,
    Right: Hash + Eq,
{
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left && self.right == other.right
    }
}

impl<Left, Right> Eq for Many2Many<Left, Right>
where
    Left: Hash + Eq,
    Right: Hash + Eq,
{
}

impl<Left, Right> Extend<(Left, Right)> for Many2Many<Left, Right>
where
    Left: Hash + Eq,
    Right: Hash + Eq,
{
    #[inline]
    fn extend<T: IntoIterator<Item = (Left, Right)>>(&mut self, iter: T) {
        iter.into_iter().for_each(move |(left, right)| {
            self.insert(left, right);
        });
    }
}

impl<'a, Left, Right> Extend<(&'a Left, &'a Right)> for Many2Many<Left, Right>
where
    Left: Hash + Eq + Copy,
    Right: Hash + Eq + Copy,
{
    #[inline]
    fn extend<T: IntoIterator<Item = (&'a Left, &'a Right)>>(&mut self, iter: T) {
        self.extend(iter.into_iter().map(|(&left, &right)| (left, right)));
    }
}

impl<'a, Left, Right> Extend<&'a (Left, Right)> for Many2Many<Left, Right>
where
    Left: Hash + Eq + Copy,
    Right: Hash + Eq + Copy,
{
    #[inline]
    fn extend<T: IntoIterator<Item = &'a (Left, Right)>>(&mut self, iter: T) {
        self.extend(iter.into_iter().map(|&(left, right)| (left, right)));
    }
}

impl<Left, Right> FromIterator<(Left, Right)> for Many2Many<Left, Right>
where
    Left: Hash + Eq,
    Right: Hash + Eq,
{
    fn from_iter<T: IntoIterator<Item = (Left, Right)>>(iter: T) -> Self {
        let mut map = Many2Many::default();
        iter.into_iter().for_each(|(left, right)| {
            map.insert(left, right);
        });
        map
    }
}

impl<Left, Right, const N: usize> From<[(Left, Right); N]> for Many2Many<Left, Right>
where
    Left: Hash + Eq,
    Right: Hash + Eq,
{
    fn from(value: [(Left, Right); N]) -> Self {
        value.into_iter().collect()
    }
}

impl<Left, Right> Default for Many2Many<Left, Right> {
    #[inline]
    fn default() -> Self {
        Many2Many {
            left: Default::default(),
            right: Default::default(),
        }
    }
}

impl<Left, Right> Many2Many<Left, Right> {
    pub fn new() -> Many2Many<Left, Right> {
        Many2Many::default()
    }

    pub fn clear(&mut self) {
        self.left.clear();
        self.right.clear();
    }
}

impl<Left, Right> Many2Many<Left, Right>
where
    Left: Hash + Eq,
    Right: Hash + Eq,
{
    pub fn insert(&mut self, left: Left, right: Right) -> bool {
        match (
            self.left.get_key_value_mut(&left),
            self.right.get_key_value_mut(&right),
        ) {
            (Some((ll, lr)), Some((rr, rl))) => {
                if lr.contains(rr) && rl.contains(ll) {
                    return false;
                }

                lr.insert(rr.clone());
                rl.insert(ll.clone());
            }
            (Some((ll, lr)), None) => {
                let rr = Ref::new(right);

                lr.insert(rr.clone());

                let mut set = HashSet::new();
                set.insert(ll.clone());
                self.right.insert(rr.clone(), set);
            }
            (None, Some((rr, rl))) => {
                let ll = Ref::new(left);

                let mut set = HashSet::new();
                set.insert(rr.clone());
                self.left.insert(ll.clone(), set);

                rl.insert(ll.clone());
            }
            (None, None) => {
                let ll = Ref::new(left);
                let rr = Ref::new(right);

                let mut set = HashSet::new();
                set.insert(rr.clone());
                self.left.insert(ll.clone(), set);

                let mut set = HashSet::new();
                set.insert(ll.clone());
                self.right.insert(rr.clone(), set);
            }
        }

        true
    }

    pub fn get_by_left(&self, left: &Left) -> Option<Vec<&Right>> {
        if let Some(set) = self.left.get(left) {
            let v = set.iter().map(|x| &(**x)).collect();
            Some(v)
        } else {
            None
        }
    }

    pub fn get_by_right(&self, right: &Right) -> Option<Vec<&Left>> {
        if let Some(set) = self.right.get(right) {
            let v = set.iter().map(|x| &(**x)).collect();
            Some(v)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Many2Many;

    #[test]
    fn test_insert() {
        let mut map = Many2Many::new();
        assert!(map.insert(1, "a"));
        assert!(map.insert(1, "b"));
        assert!(map.insert(2, "a"));
        assert!(map.insert(2, "b"));

        assert!(!map.insert(1, "a"));
    }
}
