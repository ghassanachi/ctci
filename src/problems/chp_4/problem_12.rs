use crate::structures::{BinaryTree, BinaryTreeUtil, TreeNodeRef};

/// Since the question did not clarify I am making the assumption that a single node path is
/// considered a valid path. If that were not the case, appending the node value to the path could
/// be moved after the update loop so as not to count it. The rest of the algorithm would
/// remain the same.
fn binary_path_sum_helper(
    node: TreeNodeRef<i32>,
    paths: &mut Vec<i32>,
    target_sum: i32,
    valid_paths: &mut u32,
) {
    if let Some(node) = node {
        let val = node.borrow().val;
        paths.push(0);
        for path in paths.iter_mut() {
            *path += val;
            if *path == target_sum {
                *valid_paths += 1;
            }
        }
        // Check left tree
        binary_path_sum_helper(node.left(), paths, target_sum, valid_paths);
        binary_path_sum_helper(node.right(), paths, target_sum, valid_paths);
        paths.pop();
        for path in paths.iter_mut() {
            *path -= val;
        }
    }
}

pub fn binary_path_sum(tree: &BinaryTree<i32>, target_sum: i32) -> u32 {
    let valid_paths = &mut 0u32;
    let paths = &mut Vec::<i32>::new();
    binary_path_sum_helper(
        tree.root.as_ref().map(|n| n.clone()),
        paths,
        target_sum,
        valid_paths,
    );
    *valid_paths
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn binary_path_sum_1() {
        let tree = BinaryTree::build(&[
            Some(2),
            Some(-1),
            None,
            Some(-1),
            None,
            None,
            Some(3),
            None,
            Some(4),
        ]);
        assert_eq!(binary_path_sum(&tree, 5), 1);
        assert_eq!(binary_path_sum(&tree, 0), 1);
        assert_eq!(binary_path_sum(&tree, -1), 2);
        assert_eq!(binary_path_sum(&tree, 7), 1);
        assert_eq!(binary_path_sum(&tree, 8), 0);
    }

    #[test]
    fn binary_path_sum_2() {
        let tree = BinaryTree::build(&[Some(-2), Some(2), Some(-2), Some(2)]);
        assert_eq!(binary_path_sum(&tree, 0), 4);
        assert_eq!(binary_path_sum(&tree, -2), 3);
        assert_eq!(binary_path_sum(&tree, 2), 3);
        assert_eq!(binary_path_sum(&tree, 3), 0);
    }

    #[test]
    fn binary_path_sum_3() {
        let tree = BinaryTree::new();
        assert_eq!(binary_path_sum(&tree, 0), 0);
        assert_eq!(binary_path_sum(&tree, 1), 0);
    }
}
