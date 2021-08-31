use crate::structures::{NodeActions, RBChild};
use std::fmt::Debug;

fn smallest<T: Debug>(node: RBChild<T>) -> RBChild<T> {
    if let Some(node) = node {
        if node.borrow().left.is_some() {
            return smallest(node.borrow().left.clone());
        }
        return Some(node);
    }
    None
}

/// Straightforward check for next value following inorder pattern
///
/// 1. Try to return the smallest element in the "Right" subtree,
/// 2. Return the parent (if you are the left child of the parent, thus smaller)
/// 3. Return none;
pub fn get_next(node: RBChild<i32>) -> RBChild<i32> {
    if let Some(node) = &node {
        let right = node.borrow().right.as_ref().map(|n| n.clone());
        let small_right_st = smallest(right);
        if small_right_st.is_some() {
            return small_right_st;
        }
        let mut cursor = node.clone();
        while let Some(parent) = cursor.parent() {
            if parent.is_left(cursor) {
                return Some(parent);
            }
            cursor = parent;
        }
    }
    None
}

/// Empty test cases since the testing is already done in the `BinaryTree<T>` module
#[cfg(test)]
mod tests {

    use super::*;
    use crate::structures::BinaryTreeValidator;
    use crate::structures::RBTree;
    use rand::*;

    /// Generate random vector of i32 of size `size`
    fn generate_random_vec(size: u32) -> Vec<i32> {
        let mut rng = rand::thread_rng();
        let mut items = Vec::with_capacity(size as usize);
        for _ in 0..size {
            let random = rng.gen_range(0..size) as i32;
            items.push(random as i32);
        }
        items
    }
    /// Fill the tree with items and then return a random node and its expected next value
    ///
    /// Important: It's important to make sure to know when Rc will be dropped otherwise parts of
    /// the tree can "dissapear" while you are still traversing it. This is what was happening
    /// prior to passing the tree as a &mut
    fn prepare_run(tree: &mut RBTree<i32>, mut items: Vec<i32>) -> (RBChild<i32>, Option<i32>) {
        let mut rng = rand::thread_rng();
        for &i in items.iter() {
            tree.insert(i);
        }
        let nodes = tree.nodes();
        let random_node = rng.gen_range(0..nodes.len());
        let random_node = &nodes[random_node];
        let node_val = random_node.val();

        items.sort();
        items.dedup();

        let pos = items.binary_search(&node_val).unwrap();

        let expected_value = if pos != items.len() - 1 {
            Some(items[pos + 1])
        } else {
            None
        };
        (Some(random_node.clone()), expected_value)
    }

    #[test]
    fn get_next_1() {
        let items = vec![1, 2, 3, 4, 5];
        let tree = RBTree::from(items);
        let nodes = tree.nodes();
        let node_pos = nodes.binary_search_by(|node| 3.cmp(&node.val())).unwrap();
        let random_node = nodes[node_pos].clone();
        let next = get_next(Some(random_node)).unwrap();
        assert_eq!(next.val(), 4);
    }

    #[test]
    fn get_next_2() {
        let mut items: Vec<_> = vec![10, 5, 8, 0, 3, 2, 0, 1];
        items.sort();
        items.dedup();

        println!("{:?}", items);
        let tree = RBTree::from(items);
        let nodes = tree.nodes();
        let node_pos = nodes.binary_search_by(|node| node.val().cmp(&0)).unwrap();
        let random_node = nodes[node_pos].clone();
        let next = get_next(Some(random_node)).unwrap();
        assert_eq!(next.val(), 1);
    }

    #[test]
    fn get_next_3() {
        for size in vec![1, 10, 100, 1000] {
            let items = generate_random_vec(size);
            let mut tree = RBTree::from(items.clone());
            let (node, expected_value) = prepare_run(&mut tree, items);
            let next = get_next(node);
            if let Some(next) = next {
                assert_eq!(next.val(), expected_value.unwrap());
            } else {
                assert!(expected_value.is_none())
            }
        }
    }
}
