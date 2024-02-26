use std::{fmt::Debug, hash::Hash};

use hashbrown::{HashMap, HashSet};

mod into_lefts;
mod into_rights;
mod lefts;
mod rights;

#[cfg(feature = "thread_safe")]
pub type Ref<T> = std::sync::Arc<T>;
#[cfg(not(feature = "thread_safe"))]
pub type Ref<T> = std::rc::Rc<T>;

#[derive(Debug)]
pub struct Many2Many<Left, Right>
where
    Left: Hash + Eq + Clone,
    Right: Hash + Eq + Clone,
{
    left: HashMap<Ref<Left>, HashSet<Ref<Right>>>,
    right: HashMap<Ref<Right>, HashSet<Ref<Left>>>,
}

impl<Left, Right> Many2Many<Left, Right>
where
    Left: Hash + Eq + Clone,
    Right: Hash + Eq + Clone,
{
    fn insert_left(&mut self, left: &Ref<Left>, right: &Ref<Right>) -> bool {
        let right = if self.right.contains_key(right) {
            self.right.keys().find(|x| *x == right).unwrap().clone()
        } else {
            right.clone()
        };

        if !self.left.contains_key(left) {
            self.left.insert(left.clone(), HashSet::new());
        }

        self.left.get_mut(left).unwrap().insert(right);

        true
    }

    fn insert_right(&mut self, left: &Ref<Left>, right: &Ref<Right>) -> bool {
        let left = if self.left.contains_key(left) {
            self.left.keys().find(|x| *x == left).unwrap().clone()
        } else {
            left.clone()
        };

        if !self.right.contains_key(right) {
            self.right.insert(right.clone(), HashSet::new());
        }

        self.right.get_mut(right).unwrap().insert(left);

        true
    }
}

impl<Left, Right> Many2Many<Left, Right>
where
    Left: Hash + Eq + Clone,
    Right: Hash + Eq + Clone,
{
    pub fn new() -> Many2Many<Left, Right> {
        Many2Many {
            left: HashMap::new(),
            right: HashMap::new(),
        }
    }

    pub fn insert(&mut self, left: Left, right: Right) -> bool {
        if self.left.contains_key(&left) && self.right.contains_key(&right) {
            return false;
        }

        let left = Ref::new(left);
        let right = Ref::new(right);

        self.insert_left(&left, &right) && self.insert_right(&left, &right)
    }

    pub fn clear(&mut self) {
        self.left.clear();
        self.right.clear();
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
