use crate::structures::BinaryTreeUtil;
use crate::structures::{BinaryTree, TreeNodeRef};
use std::fmt::Debug;

fn has_subtree_helper<T: Eq + Debug>(root: TreeNodeRef<T>, sub_root: TreeNodeRef<T>) -> bool {
    match (&root, &sub_root) {
        (None, None) => return true,
        (Some(_), None) => return true,
        (None, Some(_)) => return false,
        _ => {}
    }

    let sub_root = sub_root.unwrap();
    let mut potential_nodes: Vec<TreeNodeRef<T>> = Vec::new();
    let mut dfs_stack = vec![root.as_ref().map(|n| n.clone())];
    while let Some(element) = dfs_stack.pop() {
        if element.is_none() {
            continue;
        }
        let node = element.unwrap();
        if node.borrow().val == sub_root.borrow().val {
            potential_nodes.push(Some(node.clone()));
        }
        dfs_stack.push(node.left());
        dfs_stack.push(node.right());
    }

    for node in potential_nodes {
        if is_subtree(Some(sub_root.clone()), node) {
            return true;
        }
    }
    false
}

fn is_subtree<T: Eq + Debug>(root: TreeNodeRef<T>, node: TreeNodeRef<T>) -> bool {
    match (root, node) {
        (Some(_), None) => return false,
        (None, Some(_)) => return false,
        (Some(root), Some(node)) => {
            if root.borrow().val != node.borrow().val {
                return false;
            }
            return is_subtree(root.left(), node.left()) && is_subtree(root.right(), node.right());
        }
        (None, None) => return true,
    }
}

pub fn has_subtree<T: Eq + Debug>(tree: &BinaryTree<T>, subtree: &BinaryTree<T>) -> bool {
    let tree_node = tree.root.as_ref().map(|n| n.clone());
    let subtree_node = subtree.root.as_ref().map(|n| n.clone());
    has_subtree_helper(tree_node, subtree_node)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn has_subtree_1() {
        let tree = BinaryTree::build(&vec![Some(2), Some(3), None, None, Some(4)]);
        let subtree = BinaryTree::build(&vec![Some(1), Some(3), None, None, Some(4)]);
        assert!(!has_subtree(&tree, &subtree));
        let subtree = BinaryTree::build(&vec![Some(2)]);
        assert!(!has_subtree(&tree, &subtree));
        let subtree = BinaryTree::build(&vec![Some(3)]);
        assert!(has_subtree(&tree, &subtree));
        let subtree = BinaryTree::build(&vec![Some(2), Some(3), None, None, Some(4)]);
        assert!(has_subtree(&tree, &subtree));
    }

    #[test]
    fn has_subtree_2() {
        let tree = BinaryTree::build(&vec![Some(2), Some(3), None, None, Some(4)]);
        let subtree = BinaryTree::<i32>::build(&vec![]);
        assert!(has_subtree(&tree, &subtree));
        assert!(!has_subtree(&subtree, &tree));
    }

    #[test]
    fn has_subtree_3() {
        let tree = BinaryTree::build(&vec![
            Some(4),
            Some(2),
            Some(1),
            None,
            None,
            Some(3),
            None,
            None,
            Some(6),
            Some(5),
            None,
            None,
            Some(7),
        ]);
        let subtree = BinaryTree::build(&vec![Some(6), Some(5), None, None, Some(7)]);
        tree.print();
        subtree.print();
        assert!(has_subtree(&tree, &subtree));
    }
}
