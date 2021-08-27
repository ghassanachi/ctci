use crate::structures::{BinaryTreePrint, BinaryTreeUtil, BinaryTreeValidator};
use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

/// Very rough implementation of a BinaryTree.
///
/// Add some helper/conveninence methods as needed while working on Chp4 problem 2 through 5.
/// I'll most likely implement a Rc / RefCell implementation of a Binary Tree for future problems
/// certain algorithms (like bfs) don't play well with the Borrow checker.
///
/// I'll also likely implement one of the more advanced BST (either AVL or Red/Black for practice
/// purposes)

#[derive(Debug)]
pub struct BinaryTree<T> {
    pub root: NodeRef<T>,
}

type BareNode<T> = Rc<RefCell<Node<T>>>;
pub type NodeRef<T> = Option<BareNode<T>>;

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
            Some(Rc::new(RefCell::new(Node {
                val: *val,
                left: Self::build_helper(src, cur),
                right: Self::build_helper(src, cur),
            })))
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

    pub fn depth(&self) -> usize {
        if let Some(root) = &self.root {
            return root.depth();
        }
        0
    }

    pub fn is_valid_bst(&self) -> bool
    where
        T: PartialOrd + Copy,
    {
        if let Some(root) = &self.root {
            return root.is_valid_bst();
        }
        true
    }

    pub fn print(&self)
    where
        T: Display,
    {
        if let Some(root) = &self.root {
            return root.pprint();
        }
        println!("Empty Tree");
    }
}

impl<T> BinaryTreeUtil for BareNode<T> {
    fn left(&self) -> Option<Self> {
        self.borrow().left.as_ref().map(|n| n.clone())
    }

    fn right(&self) -> Option<Self> {
        self.borrow().right.as_ref().map(|n| n.clone())
    }
}

impl<T> BinaryTreePrint<T> for BareNode<T>
where
    T: Display,
{
    fn print_node(&self) -> String {
        format!("{}", self.borrow().val)
    }
}

impl<T> BinaryTreeValidator<T> for BareNode<T>
where
    T: PartialOrd + Copy,
{
    fn val(&self) -> T {
        self.borrow().val
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
    }

    #[test]
    fn binary_tree_basic_validate_bst() {
        //     Valid tree:
        //          4
        //        /   \
        //       2     6
        //      / \   / \
        //     1   3 5   7
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
        assert!(tree.is_valid_bst());

        //    Invalid tree 1:
        //          4
        //        /   \
        //       2     7
        //      / \   / \
        //     1   5 6   8
        let tree = BinaryTree::build(&vec![
            Some(4),
            Some(2),
            Some(1),
            None,
            None,
            Some(5),
            None,
            None,
            Some(7),
            Some(6),
            None,
            None,
            Some(8),
        ]);
        assert!(!tree.is_valid_bst());

        //    Invalid tree 2:
        //          4
        //        /   \
        //       2     6
        //      / \   / \
        //     1   3 2   7
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
            Some(2),
            None,
            None,
            Some(7),
        ]);
        assert!(!tree.is_valid_bst());

        //     Valid tree: duplicates should be stored on right branch
        //          4
        //        /   \
        //       2     6
        //      / \   / \
        //     1   3 4   7
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
            Some(4),
            None,
            None,
            Some(7),
        ]);
        assert!(tree.is_valid_bst());

        //     Invalid tree: duplicates should be stored on right branch
        //          4
        //        /   \
        //       2     6
        //      / \   / \
        //     1   4 5   7
        let tree = BinaryTree::build(&vec![
            Some(4),
            Some(2),
            Some(1),
            None,
            None,
            Some(4),
            None,
            None,
            Some(6),
            Some(5),
            None,
            None,
            Some(7),
        ]);
        assert!(!tree.is_valid_bst());
    }
}
