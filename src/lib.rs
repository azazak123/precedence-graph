pub mod create_graph;
mod node;
pub mod precedence_graph;

#[cfg(test)]
mod tests {
    use crate::precedence_graph::PrecedenceGraph;
    use std::collections::HashSet;

    mod quasi_interval_order {
        use super::*;
        use crate::create_graph;

        #[test]
        fn first_type_definition() {
            let g = create_graph!(
                Nodes: [],
                Edges: [1 => 2, 1 => 3, 4 => 5]
            )
            .unwrap();

            assert!(g.is_first());
            assert!(!g.is_quasi_interval_order());
        }

        #[test]
        fn second_type_definition() {
            let g = create_graph!(
                Nodes: [],
                Edges: [1 => 3, 2 => 3, 4 => 5]
            )
            .unwrap();

            assert!(g.is_second());
            assert!(!g.is_quasi_interval_order());
        }

        #[test]
        fn third_type_definition() {
            let g = create_graph!(
                Nodes: [],
                Edges: [1 => 2, 1 => 3, 4 => 5, 4 => 3, 5 => 3]
            )
            .unwrap();

            assert!(g.is_third());
            assert!(!g.is_quasi_interval_order());
        }

        #[test]
        fn is_quasi_interval_order() {
            let g = create_graph!(
                Nodes: [],
                Edges: [1 => 2, 4 => 5]
            )
            .unwrap();

            assert!(!g.is_first());
            assert!(!g.is_second());
            assert!(!g.is_third());
            assert!(g.is_quasi_interval_order());

            let g = create_graph!(
                Nodes: [],
                Edges: [1 => 4, 1 => 6, 2 => 4, 2 => 5, 2 => 6, 3 => 6, 5 => 7, 6 => 7]
            )
            .unwrap();

            assert!(!g.is_first());
            assert!(!g.is_second());
            assert!(!g.is_third());
            assert!(g.is_quasi_interval_order());

            let g = create_graph!(
                Nodes: [],
                Edges: [1 => 3, 2 => 3, 3 => 5, 4 => 5]
            )
            .unwrap();

            assert!(!g.is_first());
            assert!(!g.is_second());
            assert!(!g.is_third());
            assert!(g.is_quasi_interval_order());

            let g = create_graph!(
                Nodes: [],
                Edges: [1 => 4, 2 => 4, 3 => 4, 4 => 5, 4 => 6]
            )
            .unwrap();

            assert!(!g.is_first());
            assert!(!g.is_second());
            assert!(!g.is_third());
            assert!(g.is_quasi_interval_order());

            let g = create_graph!(
                Nodes: [],
                Edges: [1 => 4, 2 => 4, 2 => 3, 4 => 5, 3 => 5]
            )
            .unwrap();

            assert!(!g.is_first());
            assert!(!g.is_second());
            assert!(!g.is_third());
            assert!(g.is_quasi_interval_order());
        }

        #[test]
        fn is_not_quasi_interval_order() {
            let g = create_graph!(
                Nodes: [],
                Edges: [5 => 1, 5 => 2, 5 => 3, 7 => 6, 6 => 4]
            )
            .unwrap();

            assert!(g.is_first());
            assert!(!g.is_second());
            assert!(!g.is_third());
            assert!(!g.is_quasi_interval_order());

            let g = create_graph!(
                Nodes: [],
                Edges: [8 => 5, 5 => 1, 5 => 2, 5 => 3, 7 => 6, 6 => 4]
            )
            .unwrap();

            assert!(g.is_first());
            assert!(!g.is_second());
            assert!(!g.is_third());
            assert!(!g.is_quasi_interval_order());

            let g = create_graph!(
                Nodes: [],
                Edges: [10 => 5, 10 => 8, 9 => 8, 8 => 6, 6 => 4, 4 => 2, 7 => 5, 7 => 4, 5 => 3, 3 => 1]
            )
            .unwrap();

            assert!(!g.is_first());
            assert!(!g.is_second());
            assert!(g.is_third());
            assert!(!g.is_quasi_interval_order());

            let g = create_graph!(
                Nodes: [],
                Edges: [1 => 3, 2 => 3, 3 => 5, 4 => 6, 6 => 5]
            )
            .unwrap();

            assert!(!g.is_first());
            assert!(g.is_second());
            assert!(!g.is_third());
            assert!(!g.is_quasi_interval_order());

            let g = create_graph!(
                Nodes: [],
                Edges: [1 => 3, 1 => 5, 2 => 3, 2 => 6, 5 => 7, 6 => 7, 3 => 8, 7 => 9, 8 => 9]
            )
            .unwrap();

            assert!(!g.is_first());
            assert!(g.is_second());
            assert!(!g.is_third());
            assert!(!g.is_quasi_interval_order());

            let g = create_graph!(
                Nodes: [],
                Edges: [1 => 2, 1 => 3, 2 => 4, 4 => 7, 7 => 8, 3 => 5, 3 => 6, 5 => 8, 6 => 8]
            )
            .unwrap();

            assert!(g.is_first());
            assert!(!g.is_second());
            assert!(!g.is_third());
            assert!(!g.is_quasi_interval_order());

            let g = create_graph!(
                Nodes: [],
                Edges: [1 => 2, 1 => 3, 2 => 4, 4 => 7, 7 => 10, 3 => 5, 5 => 6, 6 => 8, 6 => 9, 8 => 10, 9 => 10]
            )
            .unwrap();

            assert!(g.is_first());
            assert!(!g.is_second());
            assert!(!g.is_third());
            assert!(!g.is_quasi_interval_order());
        }
    }

    mod linear_order {
        use super::*;
        use crate::create_graph;

        #[test]
        fn is_linear_order() {
            let g = create_graph!(
                Nodes: [],
                Edges: [1 => 2, 2 => 3]
            )
            .unwrap();

            assert!(g.is_linear_order());

            let g = create_graph!(
                Nodes: [],
                Edges: []
            )
            .unwrap();

            assert!(g.is_linear_order());
        }

        #[test]
        fn is_not_linear_order() {
            let g = create_graph!(
                Nodes: [],
                Edges: [1 => 2, 4 => 5]
            )
            .unwrap();

            assert!(!g.is_linear_order());
        }
    }

    mod msf {
        use super::*;
        use crate::create_graph;

        #[test]
        fn general() {
            let g = create_graph!(
                Nodes: [],
                Edges: [5 => 1, 5 => 2, 5 => 3, 7 => 6, 6 => 4]
            )
            .unwrap();

            let res = g.msf_list();

            assert!(matches!(res[0], 5));
            assert!(matches!(res[1], 7));
            assert!(matches!(res[2], 6));
            assert!(matches!(res[3], 1 | 2 | 3 | 4));
            assert!(matches!(res[4], 1 | 2 | 3 | 4));
            assert!(matches!(res[5], 1 | 2 | 3 | 4));
            assert!(matches!(res[6], 1 | 2 | 3 | 4));

            let g = create_graph!(
                Nodes: [],
                Edges: [8 => 5, 5 => 1, 5 => 2, 5 => 3, 7 => 6, 6 => 4]
            )
            .unwrap();

            let res = g.msf_list();

            assert!(matches!(res[0], 8));
            assert!(matches!(res[1], 5 | 7));
            assert!(matches!(res[2], 5 | 7));
            assert!(matches!(res[3], 6));
            assert!(matches!(res[4], 1 | 2 | 3 | 4));
            assert!(matches!(res[5], 1 | 2 | 3 | 4));
            assert!(matches!(res[6], 1 | 2 | 3 | 4));
            assert!(matches!(res[7], 1 | 2 | 3 | 4));

            let g = create_graph!(
                Nodes: [],
                Edges: [1 => 3, 2 => 3, 3 => 5, 4 => 6, 6 => 5]
            )
            .unwrap();

            let res = g.msf_list();

            assert!(matches!(res[0], 1 | 2 | 4));
            assert!(matches!(res[1], 1 | 2 | 4));
            assert!(matches!(res[2], 1 | 2 | 4));
            assert!(matches!(res[3], 3 | 6));
            assert!(matches!(res[4], 3 | 6));
            assert!(matches!(res[5], 5));

            let g = create_graph!(
                Nodes: [],
                Edges: [1 => 3, 1 => 5, 2 => 3, 2 => 6, 5 => 7, 6 => 7, 3 => 8, 7 => 9, 8 => 9]
            )
            .unwrap();

            let res = g.msf_list();

            assert!(matches!(res[0], 1 | 2));
            assert!(matches!(res[1], 1 | 2));
            assert!(matches!(res[2], 3 | 5 | 6));
            assert!(matches!(res[3], 3 | 5 | 6));
            assert!(matches!(res[4], 3 | 5 | 6));
            assert!(matches!(res[5], 7 | 8));
            assert!(matches!(res[6], 7 | 8));
            assert!(matches!(res[7], 9));

            let g = create_graph!(
                Nodes: [],
                Edges: [1 => 2, 1 => 3, 2 => 4, 4 => 7, 7 => 8, 3 => 5, 3 => 6, 5 => 8, 6 => 8]
            )
            .unwrap();

            let res = g.msf_list();

            assert!(matches!(res[0], 1));
            assert!(matches!(res[1], 2 | 3));
            assert!(matches!(res[2], 2 | 3));
            assert!(matches!(res[3], 4));
            assert!(matches!(res[4], 5 | 6 | 7));
            assert!(matches!(res[5], 5 | 6 | 7));
            assert!(matches!(res[6], 5 | 6 | 7));
            assert!(matches!(res[7], 8));

            let g = create_graph!(
                Nodes: [],
                Edges: [1 => 2, 1 => 3, 2 => 4, 4 => 7, 7 => 10, 3 => 5, 5 => 6, 6 => 8, 6 => 9, 8 => 10, 9 => 10]
            )
            .unwrap();

            let res = g.msf_list();

            assert!(matches!(res[0], 1));
            assert!(matches!(res[1], 3));
            assert!(matches!(res[2], 5));
            assert!(matches!(res[3], 2 | 6));
            assert!(matches!(res[4], 2 | 6));
            assert!(matches!(res[5], 4));
            assert!(matches!(res[6], 7 | 8 | 9));
            assert!(matches!(res[7], 7 | 8 | 9));
            assert!(matches!(res[8], 7 | 8 | 9));
            assert!(matches!(res[9], 10));
        }

        #[test]
        fn quasi_interval_order() {
            let g = create_graph!(
                Nodes: [],
                Edges: [1 => 2, 4 => 5]
            )
            .unwrap();

            let res = g.msf_list();

            assert!(matches!(res[0], 1 | 4));
            assert!(matches!(res[1], 1 | 4));
            assert!(matches!(res[2], 2 | 5));
            assert!(matches!(res[2], 2 | 5));

            let g = create_graph!(
                Nodes: [],
                Edges: [1 => 4, 1 => 6, 2 => 4, 2 => 5, 2 => 6, 3 => 6, 5 => 7, 6 => 7]
            )
            .unwrap();

            let res = g.msf_list();

            assert!(matches!(res[0], 2));
            assert!(matches!(res[1], 1));
            assert!(matches!(res[2], 3));
            assert!(matches!(res[3], 5 | 6));
            assert!(matches!(res[4], 5 | 6));
            assert!(matches!(res[5], 4 | 7));
            assert!(matches!(res[6], 4 | 7));

            let g = create_graph!(
                Nodes: [],
                Edges: [1 => 3, 2 => 3, 3 => 5, 4 => 5]
            )
            .unwrap();

            let res = g.msf_list();

            assert!(matches!(res[0], 1 | 2));
            assert!(matches!(res[1], 1 | 2));
            assert!(matches!(res[2], 3 | 4));
            assert!(matches!(res[3], 3 | 4));
            assert!(matches!(res[4], 5));

            let g = create_graph!(
                Nodes: [],
                Edges: [1 => 4, 2 => 4, 3 => 4, 4 => 5, 4 => 6]
            )
            .unwrap();

            let res = g.msf_list();

            assert!(matches!(res[0], 1 | 2 | 3));
            assert!(matches!(res[1], 1 | 2 | 3));
            assert!(matches!(res[2], 1 | 2 | 3));
            assert!(matches!(res[3], 4));
            assert!(matches!(res[4], 5 | 6));
            assert!(matches!(res[5], 5 | 6));

            let g = create_graph!(
                Nodes: [],
                Edges: [1 => 4, 2 => 4, 2 => 3, 4 => 5, 3 => 5]
            )
            .unwrap();

            let res = g.msf_list();

            assert!(matches!(res[0], 2));
            assert!(matches!(res[1], 1));
            assert!(matches!(res[2], 3 | 4));
            assert!(matches!(res[3], 3 | 4));
            assert!(matches!(res[4], 5));
        }

        #[test]
        fn overinterval() {
            let g = create_graph!(
            Nodes: [],
            Edges: [10 => 5, 10 => 8, 9 => 8, 8 => 6, 6 => 4, 4 => 2, 7 => 5, 7 => 4, 5 => 3, 3 => 1]
        )
        .unwrap();

            let res = g.msf_list();

            assert!(matches!(res[0], 10));
            assert!(matches!(res[1], 7));
            assert!(matches!(res[2], 9));
            assert!(matches!(res[3], 8));
            assert!(matches!(res[4], 5 | 6));
            assert!(matches!(res[5], 5 | 6));
            assert!(matches!(res[6], 3 | 4));
            assert!(matches!(res[7], 3 | 4));
            assert!(matches!(res[8], 1 | 2));
            assert!(matches!(res[9], 1 | 2));
        }
    }

    mod gs {
        use super::*;
        use crate::create_graph;

        #[test]
        fn quasi_interval_order() {
            let g = create_graph!(
                Nodes: [],
                Edges: [1 => 2, 4 => 5]
            )
            .unwrap();

            let res = g.gc_list();

            assert!(matches!(res[0], 1 | 4));
            assert!(matches!(res[1], 1 | 4));
            assert!(matches!(res[2], 2 | 5));
            assert!(matches!(res[2], 2 | 5));

            let g = create_graph!(
                Nodes: [],
                Edges: [1 => 4, 1 => 6, 2 => 4, 2 => 5, 2 => 6, 3 => 6, 5 => 7, 6 => 7]
            )
            .unwrap();

            let res = g.gc_list();

            assert!(matches!(res[0], 2));
            assert!(matches!(res[1], 1));
            assert!(matches!(res[2], 3));
            assert!(matches!(res[3], 5 | 6));
            assert!(matches!(res[4], 5 | 6));
            assert!(matches!(res[5], 4 | 7));
            assert!(matches!(res[6], 4 | 7));

            let g = create_graph!(
                Nodes: [],
                Edges: [1 => 3, 2 => 3, 3 => 5, 4 => 5]
            )
            .unwrap();

            let res = g.gc_list();

            assert!(matches!(res[0], 1 | 2));
            assert!(matches!(res[1], 1 | 2));
            assert!(matches!(res[2], 3 | 4));
            assert!(matches!(res[3], 3 | 4));
            assert!(matches!(res[4], 5));

            let g = create_graph!(
                Nodes: [],
                Edges: [1 => 4, 2 => 4, 3 => 4, 4 => 5, 4 => 6]
            )
            .unwrap();

            let res = g.gc_list();

            assert!(matches!(res[0], 1 | 2 | 3));
            assert!(matches!(res[1], 1 | 2 | 3));
            assert!(matches!(res[2], 1 | 2 | 3));
            assert!(matches!(res[3], 4));
            assert!(matches!(res[4], 5 | 6));
            assert!(matches!(res[5], 5 | 6));

            let g = create_graph!(
                Nodes: [],
                Edges: [1 => 4, 2 => 4, 2 => 3, 4 => 5, 3 => 5]
            )
            .unwrap();

            let res = g.gc_list();

            assert!(matches!(res[0], 2));
            assert!(matches!(res[1], 1));
            assert!(matches!(res[2], 3 | 4));
            assert!(matches!(res[3], 3 | 4));
            assert!(matches!(res[4], 5));
        }

        #[test]
        fn gc_general() {
            let g = create_graph!(
                Nodes: [],
                Edges: [5 => 1, 5 => 2, 5 => 3, 7 => 6, 6 => 4]
            )
            .unwrap();

            let res = g.gc_list();

            assert!(matches!(res[0], 7));
            assert!(matches!(res[1], 5 | 6));
            assert!(matches!(res[2], 5 | 6));
            assert!(matches!(res[3], 1 | 2 | 3 | 4));
            assert!(matches!(res[4], 1 | 2 | 3 | 4));
            assert!(matches!(res[5], 1 | 2 | 3 | 4));
            assert!(matches!(res[6], 1 | 2 | 3 | 4));
        }

        #[test]
        fn gc_overinterval() {
            let g = create_graph!(
            Nodes: [],
            Edges: [10 => 5, 10 => 8, 9 => 8, 8 => 6, 6 => 4, 4 => 2, 7 => 5, 7 => 4, 5 => 3, 3 => 1]
        )
        .unwrap();

            let res = g.gc_list();

            assert!(matches!(res[0], 10));
            assert!(matches!(res[1], 9));
            assert!(matches!(res[2], 7 | 8));
            assert!(matches!(res[3], 7 | 8));
            assert!(matches!(res[4], 5 | 6));
            assert!(matches!(res[5], 5 | 6));
            assert!(matches!(res[6], 3 | 4));
            assert!(matches!(res[7], 3 | 4));
            assert!(matches!(res[8], 1 | 2));
            assert!(matches!(res[9], 1 | 2));
        }
    }

    mod precedence_graph {
        use super::*;
        use crate::create_graph;

        #[test]
        fn new_graph() {
            let mut nodes = HashSet::new();
            nodes.insert(5);
            nodes.insert(1);
            nodes.insert(6);
            nodes.insert(4);

            let mut edges = HashSet::new();
            edges.insert((5, 1));
            edges.insert((6, 4));

            PrecedenceGraph::new(nodes, edges).unwrap();
        }

        #[test]
        fn create_graph_macro() {
            let g1 = create_graph!(
                Nodes: [],
                Edges: [5 => 1, 6 => 4]
            )
            .unwrap();

            let mut nodes = HashSet::new();
            nodes.insert(5);
            nodes.insert(1);
            nodes.insert(6);
            nodes.insert(4);

            let mut edges = HashSet::new();
            edges.insert((5, 1));
            edges.insert((6, 4));

            let g2 = PrecedenceGraph::new(nodes, edges).unwrap();

            assert_eq!(g1, g2);
        }

        #[test]
        #[should_panic(expected = "EdgeContainsNonexistentNode { node: 10 }")]
        fn outcome_node_not_exist() {
            let mut nodes = HashSet::new();
            nodes.insert(5);
            nodes.insert(1);
            nodes.insert(6);
            nodes.insert(4);

            let mut edges = HashSet::new();
            edges.insert((5, 1));
            edges.insert((10, 4));

            PrecedenceGraph::new(nodes, edges).unwrap();
        }

        #[test]
        #[should_panic(expected = "EdgeContainsNonexistentNode { node: 10 }")]
        fn income_node_not_exist() {
            let mut nodes = HashSet::new();
            nodes.insert(5);
            nodes.insert(1);
            nodes.insert(6);
            nodes.insert(4);

            let mut edges = HashSet::new();
            edges.insert((5, 1));
            edges.insert((6, 10));

            PrecedenceGraph::new(nodes, edges).unwrap();
        }
    }
}
