use crate::structures::{BinaryTree, TreeNode, TreeNodeRef};
use std::cell::RefCell;
use std::rc::Rc;

pub fn bt_from_sorted<T: Copy>(arr: &[T]) -> BinaryTree<T> {
    let root = helper(arr);
    BinaryTree { root }
}

pub fn helper<T: Copy>(arr: &[T]) -> TreeNodeRef<T> {
    let arr_len = arr.len();
    if arr_len == 0 {
        return None;
    }
    let mid = arr_len / 2;
    let mut current = TreeNode::new(arr[mid]);
    current.left = helper(&arr[..mid]);
    current.right = helper(&arr[mid + 1..]);
    Some(Rc::new(RefCell::new(current)))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn bt_from_sorted_1() {
        let input = vec![1, 2, 3];
        let tree = bt_from_sorted(&input);
        assert!(tree.is_valid_bst());
        assert_eq!(tree.depth(), 2);
    }

    #[test]
    fn bt_from_sorted_2() {
        let input = vec![3, 2, 1];
        let tree = bt_from_sorted(&input);
        assert!(!tree.is_valid_bst());
        assert_eq!(tree.depth(), 2);
    }

    #[test]
    fn bt_from_sorted_3() {
        let input: Vec<u32> = vec![];
        let tree = bt_from_sorted(&input);
        assert!(tree.is_valid_bst());
        assert_eq!(tree.depth(), 0);
    }

    #[test]
    fn bt_from_sorted_4() {
        let input: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let tree = bt_from_sorted(&input);
        assert!(tree.is_valid_bst());
        assert_eq!(tree.depth(), 4);
    }
}
