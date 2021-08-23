use crate::structures::BinaryTree;
/// # Check if `BinaryTree<T>` is valid Binary Search Tree
///
/// Implementation is already part of my `BinaryTree<T>` API, so go to [`is_valid_bst`] to view
/// implementation details.
///
/// I added the implementation there, since it is sometimes helpful to use it for testing
/// assertions
///
/// [`is_valid_bst`]: BinaryTree::is_valid_bst
pub fn is_valid_bst<T: Copy + PartialOrd>(tree: BinaryTree<T>) -> bool {
    tree.is_valid_bst()
}

/// Empty test cases since the testing is already done in the `BinaryTree<T>` module
#[cfg(test)]
mod tests {

    #[test]
    fn bt_from_sorted_1() {
        assert!(true)
    }
}
