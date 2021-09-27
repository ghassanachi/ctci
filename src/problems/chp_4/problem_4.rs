use crate::structures::{BinaryTree, BinaryTreeUtil, TreeNodeRef};

pub fn is_balanced<T>(tree: BinaryTree<T>) -> bool {
    helper(tree.root).is_some()
}

fn helper<T>(node: TreeNodeRef<T>) -> Option<i32> {
    if let Some(node) = node {
        let l_balanced = helper(node.left());
        let r_balanced = helper(node.right());
        return match (l_balanced, r_balanced) {
            (Some(left_depth), Some(right_depth)) if (left_depth - right_depth) < 2 => {
                Some(left_depth.max(right_depth) + 1)
            }
            _ => None,
        };
    }
    Some(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_balanced_1() {
        let tree = BinaryTree::build(&vec![Some(1), Some(2), None, None, Some(3), Some(4)]);
        assert!(is_balanced(tree))
    }

    #[test]
    fn is_balanced_2() {
        let tree = BinaryTree::build(&vec![Some(1), Some(2), Some(3), None, Some(3)]);
        assert!(!is_balanced(tree))
    }

    #[test]
    fn is_balanced_3() {
        let tree: BinaryTree<i32> = BinaryTree::build(&vec![]);
        assert!(is_balanced(tree))
    }
}
