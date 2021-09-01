use crate::structures::DiGraph;
use std::collections::{HashMap, VecDeque};

#[derive(PartialEq, Eq)]
pub enum NodeState {
    NotVisisted,
    Visiting,
    Visited,
}

fn dfs(
    graph: &DiGraph<(), ()>,
    node: usize,
    visited: &mut HashMap<usize, NodeState>,
    path: &mut VecDeque<usize>,
) -> Result<(), String> {
    let state = &visited[&node];

    match state {
        NodeState::Visiting => return Err("cycle was found".to_string()),
        _ => {}
    }

    *visited.get_mut(&node).unwrap() = NodeState::Visiting;

    let neighbors = graph.neighbors(node).expect("node should exists");

    for neighbor in neighbors {
        if visited[&neighbor] == NodeState::Visited {
            continue;
        }
        dfs(&graph, neighbor, visited, path)?;
    }
    path.push_front(node);
    *visited.get_mut(&node).unwrap() = NodeState::Visited;
    Ok(())
}

fn build_graph(
    projects: &[usize],
    dependencies: &[(usize, usize)],
) -> Result<(DiGraph<(), ()>, HashMap<usize, NodeState>), String> {
    let mut visited: HashMap<usize, NodeState> = HashMap::new();
    let mut graph = DiGraph::new();
    for &node in projects {
        visited.insert(node, NodeState::NotVisisted);
        graph.add_node(node, Default::default())?;
    }
    for (from, to) in dependencies {
        graph.add_edge(*from, *to, Default::default())?;
    }
    Ok((graph, visited))
}

pub fn build_order(
    projects: Vec<usize>,
    dependencies: Vec<(usize, usize)>,
) -> Result<Vec<usize>, String> {
    let mut path = VecDeque::new();
    let (graph, mut visited) = build_graph(&projects, &dependencies)?;

    for &node in &projects {
        if visited[&node] == NodeState::Visited {
            continue;
        }
        if let Err(message) = dfs(&graph, node, &mut visited, &mut path) {
            return Err(message);
        }
    }

    Ok(path.into_iter().collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_order_simple() {
        let path = build_order(vec![1, 2, 3], vec![(3, 2), (2, 1)]);
        assert_eq!(path, Ok(vec![3, 2, 1]));
    }

    #[test]
    /// Similar to example from CTCI, with a couple more constraints to only allow 1 valid output
    /// since its easier to assert that way.
    fn build_order_complex() {
        let path = build_order(
            vec![1, 2, 3, 4, 5, 6],
            vec![(1, 4), (6, 2), (2, 4), (6, 1), (4, 3), (5, 1), (1, 2)],
        );
        assert_eq!(path, Ok(vec![6, 5, 1, 2, 4, 3]));
    }

    #[test]
    /// Impossible since there is a cycle in the dependencies (ie: there is a cycle in the graph)
    fn build_order_err() {
        let path = build_order(vec![1, 2, 3, 4], vec![(1, 4), (4, 2), (2, 3), (3, 1)]);
        assert!(path.is_err());
    }
}
