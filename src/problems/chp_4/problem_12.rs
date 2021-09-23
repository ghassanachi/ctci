use crate::structures::{BinaryTree, BinaryTreeUtil, TreeNodeRef};
use std::collections::hash_map::Entry;
use std::collections::HashMap;

/// Since the question did not clarify I am making the assumption that a single node path is
/// considered a valid path. If that were not the case, appending the node value to the path could
/// be moved after the update loop so as not to count it. The rest of the algorithm would
/// remain the same.
fn binary_path_sum_helper(
    node: TreeNodeRef<i32>,
    path_count: &mut HashMap<i32, u32>,
    running_sum: i32,
    target_sum: i32,
) -> u32 {
    if let Some(node) = node {
        let node_val = node.borrow().val;
        let running_sum = running_sum + node_val;

        let sum = running_sum - target_sum;
        let mut total_sum = *path_count.entry(sum).or_insert(0);

        if running_sum == target_sum {
            total_sum += 1;
        }

        *path_count.entry(running_sum).or_insert(0) += 1;
        total_sum += binary_path_sum_helper(node.left(), path_count, running_sum, target_sum);
        total_sum += binary_path_sum_helper(node.right(), path_count, running_sum, target_sum);
        if let Entry::Occupied(mut o) = path_count.entry(running_sum) {
            let val = o.get_mut();
            *val -= 1;
            if *val == 0 {
                o.remove_entry();
            }
        }
        return total_sum;
    }
    0
}

pub fn binary_path_sum(tree: &BinaryTree<i32>, target_sum: i32) -> u32 {
    let path_count = &mut HashMap::new();
    binary_path_sum_helper(
        tree.root.as_ref().map(|n| n.clone()),
        path_count,
        0,
        target_sum,
    )
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

    /// Example from the book (solution)
    #[test]
    fn binary_path_sum_3() {
        let tree = BinaryTree::build(&[
            Some(10),
            Some(5),
            Some(3),
            Some(3),
            None,
            None,
            Some(-2),
            None,
            None,
            Some(2),
            None,
            Some(1),
            None,
            None,
            Some(-3),
            None,
            Some(11),
        ]);
        assert_eq!(binary_path_sum(&tree, 8), 3);
    }
}
