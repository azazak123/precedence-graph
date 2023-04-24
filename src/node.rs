use std::{cell::RefCell, collections::HashSet, hash::Hash, iter, rc::Rc};

use itertools::Itertools;

#[derive(Clone, Eq, Debug)]
pub(crate) struct Node {
    pub val: u128,
    pub succ: Vec<Rc<RefCell<Node>>>,
    pub pred: Vec<Rc<RefCell<Node>>>,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

impl Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.val.hash(state);
    }
}

impl Node {
    pub fn all_successors(&self) -> impl Iterator<Item = Node> + '_ {
        self.succ.iter().flat_map(|node| {
            let node = node.borrow().clone();
            iter::once(node.clone())
                .chain(node.all_successors())
                .collect::<Vec<_>>()
        })
    }

    pub fn all_predecessor(&self) -> impl Iterator<Item = Node> + '_ {
        self.pred.iter().flat_map(|node| {
            let node = node.borrow().clone();
            iter::once(node.clone())
                .chain(node.all_predecessor())
                .collect::<Vec<_>>()
        })
    }

    pub fn comparable(&self) -> impl Iterator<Item = Node> + '_ {
        iter::once(self.clone())
            .chain(self.all_predecessor())
            .chain(self.all_successors())
            .unique()
    }

    pub fn is_succ_linear_order_without(&self, set: &HashSet<Node>) -> bool {
        let mut succ = self.succ.iter().filter(|v| !set.contains(&v.borrow()));

        if succ.clone().count() > 1 {
            return false;
        };

        let mut next = succ.next().cloned();

        while let Some(node) = next {
            let node = node.borrow();

            let mut succ = node.succ.iter().filter(|v| !set.contains(&v.borrow()));

            if succ.clone().count() > 1 {
                return false;
            }

            next = succ.next().cloned();
        }

        true
    }

    pub fn is_comparable(&self, node: &Node) -> bool {
        self.comparable().contains(node)
    }
}
