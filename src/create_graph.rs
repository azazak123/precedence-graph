#[macro_export]
macro_rules! create_graph  {
    (Nodes:[$($node:expr),*],
     Edges:[$($outcome:expr => $income:expr),*]) => {{
        #[allow(unused_mut)]
        let mut nodes = HashSet::new();
        #[allow(unused_mut)]
        let mut edges = HashSet::new();

        $(
             nodes.insert($node);
        )*

        $(
            nodes.insert($outcome);
            nodes.insert($income);

            edges.insert(($outcome, $income));
        )*

        PrecedenceGraph::new(nodes, edges)}
    };
}
