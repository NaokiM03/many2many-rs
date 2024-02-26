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
pub struct Many2Many<Left, Right> {
    left: HashMap<Ref<Left>, HashSet<Ref<Right>>>,
    right: HashMap<Ref<Right>, HashSet<Ref<Left>>>,
}

impl<Left, Right> Many2Many<Left, Right> {
    pub fn new() -> Many2Many<Left, Right> {
        Many2Many {
            left: HashMap::new(),
            right: HashMap::new(),
        }
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
