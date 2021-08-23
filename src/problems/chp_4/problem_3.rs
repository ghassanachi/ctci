use crate::structures::BinaryTree;
use crate::structures::SinglyLinkedList;
use std::collections::VecDeque;

pub fn bt_to_ll<T: Copy>(tree: BinaryTree<T>) -> Vec<SinglyLinkedList<T>> {
    let mut result = Vec::new();
    let mut queue = VecDeque::new();
    if let Some(root) = tree.root {
        queue.push_back(root);
    }
    while !queue.is_empty() {
        let mut current_level = SinglyLinkedList::new();
        let mut next_depth = VecDeque::new();
        for node in queue.drain(..) {
            current_level.append(node.val);
            if let Some(left) = node.left {
                next_depth.push_back(left);
            }
            if let Some(right) = node.right {
                next_depth.push_back(right);
            }
        }
        result.push(current_level);
        queue = next_depth;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bt_to_ll_1() {
        let tree = BinaryTree::build(&vec![Some(1), Some(2), None, None, Some(3)]);
        let lists = bt_to_ll(tree);
        assert_eq!(lists.len(), 2);
        let lvl0: Vec<_> = lists[0].values().collect();
        assert_eq!(lvl0, vec![1]);
        let lvl1: Vec<_> = lists[1].values().collect();
        assert_eq!(lvl1, vec![2, 3]);
    }

    #[test]
    fn bt_to_ll_2() {
        let tree: BinaryTree<u32> = BinaryTree::build(&Vec::new());
        let lists = bt_to_ll(tree);
        assert_eq!(lists.len(), 0);
    }

    #[test]
    fn bt_to_ll_3() {
        let tree = BinaryTree::build(&vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)]);
        let lists = bt_to_ll(tree);
        assert_eq!(lists.len(), 6);
        for i in 0..=5 {
            let lvl: Vec<_> = lists[i].values().collect();
            assert_eq!(lvl, vec![i + 1]);
        }
    }
}
