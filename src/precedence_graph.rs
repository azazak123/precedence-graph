use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use itertools::Itertools;
use thiserror::Error;

use crate::node::Node;

#[derive(Error, Debug)]
pub enum PrecedenceGraphError {
    #[error("Edge contains nonexistent node: {node}")]
    EdgeContainsNonexistentNode { node: u128 },
}

#[derive(PartialEq, Eq, Debug)]
pub struct PrecedenceGraph {
    nodes: HashSet<u128>,
    graph: HashMap<u128, Rc<RefCell<Node>>>,
}

impl PrecedenceGraph {
    pub fn new(
        nodes: HashSet<u128>,
        edges: HashSet<(u128, u128)>,
    ) -> Result<Self, PrecedenceGraphError> {
        let mut graph = HashMap::new();

        for (outcome, income) in &edges {
            if !(nodes.contains(outcome) && nodes.contains(income)) {
                return if !nodes.contains(outcome) {
                    Err(PrecedenceGraphError::EdgeContainsNonexistentNode { node: *outcome })
                } else {
                    Err(PrecedenceGraphError::EdgeContainsNonexistentNode { node: *income })
                };
            }

            let in_node = graph
                .get(income)
                .cloned()
                .unwrap_or(Rc::new(RefCell::new(Node {
                    val: *income,
                    succ: vec![],
                    pred: vec![],
                })));

            let out_node = graph
                .get(outcome)
                .cloned()
                .unwrap_or(Rc::new(RefCell::new(Node {
                    val: *outcome,
                    succ: vec![],
                    pred: vec![],
                })));

            out_node.borrow_mut().succ.push(Rc::clone(&in_node));
            in_node.borrow_mut().pred.push(Rc::clone(&out_node));

            graph.insert(*income, in_node);
            graph.insert(*outcome, out_node);
        }

        Ok(Self { nodes, graph })
    }

    pub fn is_linear_order(&self) -> bool {
        let node = match self.graph.iter().next() {
            Some((_id, node)) => node,
            None => return true,
        };

        self.nodes.len() == node.borrow().comparable().count()
    }

    pub fn is_first(&self) -> bool {
        for i in self.graph.values() {
            for j in self.graph.values() {
                let i = i.borrow();
                let j = j.borrow();

                let succ_i: HashSet<Node> = i.all_successors().collect();
                let succ_j: HashSet<Node> = j.all_successors().collect();

                if succ_i.is_subset(&succ_j) || succ_j.is_subset(&succ_i) {
                    continue;
                }

                if !i.is_succ_linear_order_without(&succ_j) {
                    return true;
                }
            }
        }

        false
    }

    pub fn is_second(&self) -> bool {
        for i in self.graph.values() {
            for j in self.graph.values() {
                let i = i.borrow();
                let j = j.borrow();

                let succ_i: HashSet<Node> = i.all_successors().collect();
                let succ_j: HashSet<Node> = j.all_successors().collect();

                for x in succ_i.difference(&succ_j) {
                    for z in succ_j.difference(&succ_i) {
                        let z_all_pred: HashSet<Node> = z.all_predecessor().collect();

                        for y in x.all_predecessor().filter(|node| !i.is_comparable(node)) {
                            if !z_all_pred.contains(&y) {
                                return true;
                            }
                        }
                    }
                }
            }
        }

        false
    }

    pub fn is_third(&self) -> bool {
        for i in self.graph.values() {
            for j in self.graph.values() {
                let i = i.borrow();
                let j = j.borrow();

                let succ_i: HashSet<Node> = i.all_successors().collect();
                let succ_j: HashSet<Node> = j.all_successors().collect();

                for x in succ_i.difference(&succ_j) {
                    for z in succ_j.difference(&succ_i) {
                        let z_all_succ: HashSet<Node> = z.all_successors().collect();

                        for y in x.all_successors().filter(|node| succ_j.contains(node)) {
                            if !z_all_succ.contains(&y) {
                                return true;
                            }
                        }
                    }
                }
            }
        }

        false
    }

    pub fn is_quasi_interval_order(&self) -> bool {
        !(self.is_first() || self.is_second() || self.is_third())
    }

    pub fn msf_list(&self) -> Vec<u128> {
        self.graph
            .values()
            .map(|node| node.borrow())
            .sorted_by(|a, b| b.all_successors().count().cmp(&a.all_successors().count()))
            .map(|node| node.val)
            .collect()
    }

    pub fn gc_list(&self) -> Vec<u128> {
        let mut j = 0;
        let n = self.nodes.len();

        let mut labels: HashMap<u128, usize> = HashMap::with_capacity(n);
        let mut list = vec![0; n];

        while j < n {
            let node = self
                .graph
                .values()
                .map(|node| node.borrow())
                .filter(|node| {
                    !labels.contains_key(&node.val)
                        && node
                            .succ
                            .iter()
                            .all(|succ| labels.contains_key(&succ.borrow().val))
                })
                .map(|node| {
                    let successors = node
                        .succ
                        .iter()
                        .map(|succ| {
                            labels
                                .get(&succ.borrow().val)
                                .expect("node should be labelled")
                        })
                        .sorted_by(|a, b| b.cmp(a))
                        .collect::<Vec<_>>();

                    (node, successors)
                })
                .min_by(|a, b| a.1.cmp(&b.1))
                .expect("iterator should have at least one node")
                .0;

            labels.insert(node.val, j);
            list[n - j - 1] = node.val;

            j += 1;
        }

        list
    }
}
