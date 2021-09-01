use crate::structures::{BinaryTreeUtil, RBBareChild, RBChild};
use std::rc::Rc;

pub fn common_ancestor<T>(
    n1: &RBBareChild<T>,
    n2: &RBBareChild<T>,
    cursor: RBChild<T>,
) -> RBChild<T> {
    if cursor.is_none() {
        return None;
    }
    let cursor_inner = cursor.as_ref().unwrap();

    if Rc::ptr_eq(n1, cursor_inner) || Rc::ptr_eq(n2, cursor_inner) {
        return cursor;
    }
    let left = common_ancestor(n1, n2, cursor_inner.left());
    let right = common_ancestor(n1, n2, cursor_inner.right());

    match (left, right) {
        (None, Some(right)) => Some(right),
        (Some(left), None) => Some(left),
        (Some(_), Some(_)) => cursor,
        (None, None) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structures::RBTree;

    fn contains<T>(root: &RBBareChild<T>, node: &RBBareChild<T>) -> bool {
        if Rc::ptr_eq(&root, &node) {
            return true;
        }
        let mut in_left = false;
        if let Some(left) = &root.left() {
            in_left = contains(left, node)
        }
        let mut in_right = false;
        if let Some(right) = &root.right() {
            in_right = contains(right, node)
        }
        in_right || in_left
    }

    #[test]
    fn common_ancestor_right() {
        let tree = RBTree::from(vec![1, 2, 3, 4, 5]);
        let nodes = tree.nodes();
        let n1 = &nodes[3];
        let n2 = &nodes[4];
        let root = tree.root.map(|n| n.clone());
        let ancestor = common_ancestor(n1, n2, root).unwrap();
        assert!(contains(&ancestor, n1));
        assert!(contains(&ancestor, n2));
    }

    #[test]
    /// Similar to example from CTCI, with a couple more constraints to only allow 1 valid output
    /// since its easier to assert that way.
    fn common_ancestor_left() {
        let tree = RBTree::from(vec![1, 2, 3, 4, 5]);
        let nodes = tree.nodes();
        let n1 = &nodes[1];
        let n2 = &nodes[2];
        let root = tree.root.map(|n| n.clone());
        let ancestor = common_ancestor(n1, n2, root).unwrap();
        assert!(contains(&ancestor, n1));
        assert!(contains(&ancestor, n2));
    }

    #[test]
    /// Impossible since there is a cycle in the dependencies (ie: there is a cycle in the graph)
    fn common_ancestor_middle() {
        let tree = RBTree::from(vec![1, 2, 3, 4, 5]);
        let nodes = tree.nodes();
        let n1 = &nodes[0];
        let n2 = &nodes[4];
        let root = tree.root.as_ref().map(|n| n.clone());
        let ancestor = common_ancestor(n1, n2, root).unwrap();
        assert!(contains(&ancestor, n1));
        assert!(contains(&ancestor, n2));
        let root = tree.root.as_ref().map(|n| n.clone()).unwrap();
        assert!(Rc::ptr_eq(&root, &ancestor));
    }
}
