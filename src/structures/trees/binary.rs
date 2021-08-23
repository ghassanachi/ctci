/// Very rough implementation of a BinaryTree.
///
/// Add some helper/conveninence methods as needed while working on Chp4 problem 2 through 5.
/// I'll most likely implement a Rc / RefCell implementation of a Binary Tree for future problems
/// certain algorithms (like bfs) don't play well with the Borrow checker.
///
/// I'll also likely implement one of the more advanced BST (either AVL or Red/Black for practice
/// purposes)
use std::fmt::{Display, Formatter, Result};

pub type NodeRef<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct Node<T> {
    pub val: T,
    pub left: NodeRef<T>,
    pub right: NodeRef<T>,
}

impl<T> Node<T> {
    pub fn new(val: T) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.val)
    }
}

#[derive(Debug)]
pub struct BinaryTree<T> {
    pub root: NodeRef<T>,
}

#[derive(Debug)]
enum NodeType {
    Left,
    Right,
    Root,
}

impl<T> BinaryTree<T> {
    fn build_helper(src: &[Option<T>], cur: &mut usize) -> NodeRef<T>
    where
        T: Copy,
    {
        if *cur >= src.len() {
            return None;
        }
        if let Some(val) = &src[*cur] {
            *cur += 1;
            Some(Box::new(Node {
                val: *val,
                left: Self::build_helper(src, cur),
                right: Self::build_helper(src, cur),
            }))
        } else {
            *cur += 1;
            None
        }
    }

    pub fn build(src: &[Option<T>]) -> Self
    where
        T: Copy,
    {
        let mut cur = 0;
        let root = Self::build_helper(src, &mut cur);
        Self { root }
    }

    fn depth_helper(node: &NodeRef<T>) -> usize {
        if let Some(node) = node {
            let left_depth = Self::depth_helper(&node.left);
            let right_depth = Self::depth_helper(&node.right);
            return std::cmp::max(left_depth, right_depth) + 1;
        }
        0
    }

    pub fn depth(&self) -> usize {
        Self::depth_helper(&self.root)
    }
}

impl<T> BinaryTree<T>
where
    T: Display,
{
    fn pprint_helper(node: &Node<T>, prefix: String, node_type: NodeType) {
        let prefix_current = "|- ";

        println!("{}{} {:?}({})", prefix, prefix_current, node_type, node);

        let prefix_child = "|  ";
        let prefix = prefix + prefix_child;

        if node.left.is_some() {
            Self::pprint_helper(
                node.left.as_deref().unwrap(),
                prefix.to_string(),
                NodeType::Left,
            );
        }
        if node.right.is_some() {
            Self::pprint_helper(
                node.right.as_deref().unwrap(),
                prefix.to_string(),
                NodeType::Right,
            );
        }
    }

    pub fn pprint(&self) {
        if let Some(root) = self.root.as_deref() {
            Self::pprint_helper(root, "".to_string(), NodeType::Root);
            return;
        }
        println!("Root: None");
    }
}

impl<T> BinaryTree<T>
where
    T: PartialOrd,
{
    fn is_valid_helper(node: &NodeRef<T>) -> bool {
        if let Some(node) = node {
            let valid = match (&node.left, &node.right) {
                (None, None) => true,
                (None, Some(right)) => right.val >= node.val,
                (Some(left), None) => left.val <= node.val,
                (Some(left), Some(right)) => right.val >= node.val && left.val <= node.val,
            };
            if !valid {
                return false;
            }
            return Self::is_valid_helper(&node.left) && Self::is_valid_helper(&node.right);
        }
        true
    }

    pub fn is_valid_bst(&self) -> bool {
        Self::is_valid_helper(&self.root)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn binary_tree_basic() {
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
        assert_eq!(tree.depth(), 3);
        assert!(tree.is_valid_bst());
    }
}
