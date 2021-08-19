/// Simple Graph implementation that uses an adjacency list.
///
/// This implementation provides both `Directed` and `UnDirected` graphs through the same Api.
/// ## Note
/// The graphs do not allow for the deletion of nodes, since this adds a significant amount of
/// complexity which is not required for the purpose of CTCI. In situations where I would need to
/// mutate the graph in this way, I will either rely on a Matrix implementation or 3rd party
/// library.
use std::collections::{hash_map::Entry, HashMap, HashSet};
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

pub trait EdgeType {
    fn is_directed() -> bool;
}

pub type NodeId = usize;

#[derive(Debug)]
pub struct Graph<N, E, Ty: EdgeType> {
    indices: HashMap<NodeId, usize>,
    nodes: Vec<Node<N>>,
    adjacencies: Vec<HashSet<Edge<E>>>,
    ty: PhantomData<Ty>,
}

#[derive(Debug)]
pub enum Directed {}

impl EdgeType for Directed {
    fn is_directed() -> bool {
        true
    }
}

#[derive(Debug)]
pub enum UnDirected {}

impl EdgeType for UnDirected {
    fn is_directed() -> bool {
        false
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Edge<E> {
    pub from: NodeId,
    pub to: NodeId,
    pub weight: E,
}

impl<E> Eq for Edge<E> {}

impl<E> PartialEq for Edge<E> {
    fn eq(&self, other: &Self) -> bool {
        self.to.eq(&other.to)
    }
}

impl<E> Hash for Edge<E> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to.hash(state)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Node<N> {
    pub index: usize,
    pub weight: N,
}

pub type DiGraph<N, E> = Graph<N, E, Directed>;
pub type UnGraph<N, E> = Graph<N, E, UnDirected>;

impl<N, E, Ty> Graph<N, E, Ty>
where
    Ty: EdgeType,
    E: Clone + Copy + Default,
    N: Clone + Copy + Default,
{
    pub fn new() -> Self {
        Self {
            indices: HashMap::new(),
            nodes: Vec::new(),
            adjacencies: Vec::new(),
            ty: PhantomData,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.len() == 0
    }

    pub fn order(&self) -> usize {
        self.nodes.len()
    }

    pub fn size(&self) -> usize {
        self.adjacencies.iter().fold(0, |acc, el| acc + el.len())
    }

    pub fn ids(&self) -> Box<dyn Iterator<Item = NodeId> + '_> {
        Box::new(self.indices.keys().map(|e| *e))
    }

    pub fn neighbors(&self, id: NodeId) -> Result<Box<dyn Iterator<Item = NodeId> + '_>, String> {
        let index = self.get_index(id)?;
        let edges = self
            .adjacencies
            .get(index)
            .ok_or(format!("node index {} does not exists", index))?;
        Ok(Box::new(edges.iter().map(|e| e.to)))
    }

    pub fn degree(&self, id: NodeId) -> Result<usize, String> {
        let index = self.get_index(id)?;
        Ok(self
            .adjacencies
            .get(index)
            .ok_or(format!("node index {} does not exists", index))?
            .len())
    }

    pub fn has_id(&self, id: NodeId) -> bool {
        return self.indices.contains_key(&id);
    }

    pub fn add_node(&mut self, id: NodeId, weight: N) -> Result<(), String> {
        let index = self.nodes.len();
        match self.indices.entry(id) {
            Entry::Occupied(_) => Err(format!("id {} already in use", id)),
            Entry::Vacant(entry) => {
                entry.insert(index);
                self.nodes.push(Node { index, weight });
                self.adjacencies.push(HashSet::new());
                Ok(())
            }
        }
    }

    pub fn is_directed(&self) -> bool {
        Ty::is_directed()
    }

    pub fn add_edge(&mut self, from: NodeId, to: NodeId, weight: E) -> Result<(), String> {
        let from_idx = self.get_index(from)?;
        let to_idx = self.get_index(to)?;

        self.adjacencies[from_idx].insert(Edge { from, to, weight });

        if !self.is_directed() {
            self.adjacencies[to_idx].insert(Edge {
                from: to,
                to: from,
                weight,
            });
        }
        Ok(())
    }

    pub fn node_edges(&self, id: NodeId) -> Result<Box<dyn Iterator<Item = Edge<E>> + '_>, String> {
        let index = self.get_index(id)?;
        let out = self.adjacencies[index].iter().map(|e| *e);
        Ok(Box::new(out))
    }

    pub fn edges(&self) -> Box<dyn Iterator<Item = Edge<E>> + '_> {
        let edges = self.adjacencies.iter().flatten().map(|e| *e);
        Box::new(edges)
    }

    pub fn has_edge(&self, from: NodeId, to: NodeId) -> Result<bool, String> {
        let from_index = self.get_index(from)?;
        let _ = self.get_index(to)?;
        Ok(self.adjacencies[from_index].contains(&Edge {
            from,
            to,
            weight: Default::default(),
        }))
    }

    pub fn from_edges(iter: &[(NodeId, NodeId)]) -> Self {
        let mut g = Self::new();
        for (from, to) in iter {
            // ignore errors adding node
            g.add_node(*from, Default::default()).ok();
            g.add_node(*to, Default::default()).ok();
            g.add_edge(*from, *to, Default::default())
                .expect(&format!("could not create edge {} => {}", from, to));
        }
        g
    }

    fn get_index(&self, id: NodeId) -> Result<usize, String> {
        Ok(*self
            .indices
            .get(&id)
            .ok_or(format!("node {} does not exists", id))?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering;

    #[test]
    fn graph_basic() {
        let graph = UnGraph::<(), ()>::from_edges(&[(1, 2), (2, 3), (3, 1)]);
        assert_eq!(graph.has_edge(1, 2), Ok(true));
        assert_eq!(graph.has_edge(2, 1), Ok(true));
        assert_eq!(graph.has_edge(2, 3), Ok(true));
        assert!(graph.has_edge(1, 4).is_err());
        assert!(graph.has_edge(4, 1).is_err());
        let mut edges: Vec<_> = graph.node_edges(1).unwrap().map(|e| e.to).collect();
        edges.sort();
        assert_eq!(edges, vec![2, 3]);

        let graph = DiGraph::<(), ()>::from_edges(&[(1, 2), (2, 3), (3, 1)]);
        assert_eq!(graph.has_edge(1, 2), Ok(true));
        assert_eq!(graph.has_edge(2, 1), Ok(false));
        assert_eq!(graph.has_edge(2, 3), Ok(true));
        assert!(graph.has_edge(1, 4).is_err());
        assert!(graph.has_edge(4, 1).is_err());
        let mut edges: Vec<_> = graph.node_edges(1).unwrap().map(|e| e.to).collect();
        edges.sort();
        assert_eq!(edges, vec![2]);
    }

    #[test]
    fn graph_helpers_undirected() {
        let graph = UnGraph::<(), ()>::new();
        assert!(graph.is_empty());

        let graph = UnGraph::<(), ()>::from_edges(&[(1, 2), (2, 3), (3, 1)]);
        assert!(!graph.is_empty());
        assert_eq!(graph.size(), 6);
        assert_eq!(graph.order(), 3);
        assert!(!graph.is_directed());
        assert!(graph.has_id(1));
        assert!(!graph.has_id(4));
        assert_eq!(graph.degree(1), Ok(2));
        assert!(graph.degree(4).is_err());

        let mut ids: Vec<_> = graph.ids().collect();
        ids.sort();
        assert_eq!(ids, vec![1, 2, 3]);
        let mut edges: Vec<_> = graph.edges().map(|e| (e.from, e.to)).collect();
        edges.sort_by(|a, b| match a.0.cmp(&b.0) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => a.1.cmp(&b.1),
            Ordering::Greater => Ordering::Greater,
        });
        assert_eq!(edges, vec![(1, 2), (1, 3), (2, 1), (2, 3), (3, 1), (3, 2)]);
    }

    #[test]
    fn graph_helpers_directed() {
        let graph = DiGraph::<(), ()>::new();
        assert!(graph.is_empty());

        let graph = DiGraph::<(), ()>::from_edges(&[(1, 2), (2, 3), (3, 1)]);
        assert!(!graph.is_empty());
        assert_eq!(graph.size(), 3);
        assert_eq!(graph.order(), 3);
        assert!(graph.is_directed());
        assert!(graph.has_id(1));
        assert!(!graph.has_id(4));
        assert_eq!(graph.degree(1), Ok(1));
        assert!(graph.degree(4).is_err());

        let mut ids: Vec<_> = graph.ids().collect();
        ids.sort();
        assert_eq!(ids, vec![1, 2, 3]);
        let mut edges: Vec<_> = graph.edges().map(|e| (e.from, e.to)).collect();
        edges.sort_by(|a, b| match a.0.cmp(&b.0) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => a.1.cmp(&b.1),
            Ordering::Greater => Ordering::Greater,
        });
        assert_eq!(edges, vec![(1, 2), (2, 3), (3, 1)]);
    }
}
