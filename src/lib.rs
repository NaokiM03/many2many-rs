use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    rc::Rc,
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Many2Many<Left, Right>
where
    Left: Hash + Eq + Clone,
    Right: Hash + Eq + Clone,
{
    left: HashMap<Rc<Left>, HashSet<Rc<Right>>>,
    right: HashMap<Rc<Right>, HashSet<Rc<Left>>>,
}
