use crate::structures::{EdgeType, Graph, NodeId};
use std::collections::HashSet;

fn dfs<Ty: EdgeType>(
    g: &Graph<(), (), Ty>,
    start: NodeId,
    end: NodeId,
    visited: &mut HashSet<NodeId>,
) -> bool {
    if start == end {
        return true;
    }

    visited.insert(start);

    let neighbors = g.neighbors(start).expect("graph is immutable");

    for node in neighbors {
        if visited.contains(&node) {
            continue;
        }
        if dfs(g, node, end, visited) {
            return true;
        }
    }
    false
}

pub fn has_route<Ty: EdgeType>(g: &Graph<(), (), Ty>, start: NodeId, end: NodeId) -> bool {
    dfs(g, start, end, &mut HashSet::new())
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::structures::{DiGraph, UnGraph};

    #[test]
    fn has_route_1() {
        let graph = DiGraph::<(), ()>::from_edges(&[(1, 2)]);
        assert_eq!(has_route(&graph, 1, 2), true);
    }

    #[test]
    fn has_route_2() {
        let graph = DiGraph::<(), ()>::from_edges(&[(1, 2), (2, 3), (3, 4)]);
        assert_eq!(has_route(&graph, 1, 4), true);

        let graph = DiGraph::<(), ()>::from_edges(&[(1, 2), (3, 2), (3, 4)]);
        assert_eq!(has_route(&graph, 1, 4), false);
    }

    #[test]
    // Added undirected graph test for fun.
    fn has_route_3() {
        let graph = UnGraph::<(), ()>::from_edges(&[(1, 2), (2, 3), (3, 4)]);
        assert_eq!(has_route(&graph, 1, 4), true);

        let graph = UnGraph::<(), ()>::from_edges(&[(1, 2), (3, 2), (3, 4)]);
        assert_eq!(has_route(&graph, 1, 4), true);
    }
}
