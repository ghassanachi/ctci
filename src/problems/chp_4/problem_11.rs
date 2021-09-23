use crate::structures::{BinaryTree, BinaryTreeUtil};
use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::fmt::Display;

#[derive(Copy, Clone)]
pub struct DepthVal<T>
where
    T: Display + Copy + Clone,
{
    val: T,
    prob: u32,
}

pub type RandomTree<T> = BinaryTree<DepthVal<T>>;

impl<T> Display for DepthVal<T>
where
    T: Display + Copy + Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "val: {} prob: {}", self.val, self.prob)
    }
}
impl<T> PartialEq for DepthVal<T>
where
    T: Eq + PartialEq + Display + Copy + Clone,
{
    fn eq(&self, other: &Self) -> bool {
        self.val.eq(&other.val)
    }
}

impl<T> Eq for DepthVal<T> where T: Display + PartialEq + Eq + Copy + Clone {}

impl<T> PartialOrd for DepthVal<T>
where
    T: PartialOrd + Display + PartialEq + Eq + Copy + Clone,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.val.partial_cmp(&other.val)
    }
}

impl<T> Ord for DepthVal<T>
where
    T: Ord + Display + Copy + Clone,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.val.cmp(&other.val)
    }
}

impl<T> RandomTree<T>
where
    T: Display + Eq + PartialEq + PartialOrd + Ord + Copy + Clone,
{
    pub fn insert_random(&mut self, val: T) {
        let mut cursor = self.root.as_ref().map(|n| n.clone());
        while let Some(node) = cursor {
            node.borrow_mut().val.prob += 1;
            let node_val = node.borrow().val.val;
            match val.cmp(&node_val) {
                Ordering::Less => {
                    if node.left().is_none() {
                        node.borrow_mut().left = Self::new_node(DepthVal { val, prob: 1 });
                        return;
                    }
                    cursor = node.left();
                }
                _ => {
                    if node.right().is_none() {
                        node.borrow_mut().right = Self::new_node(DepthVal { val, prob: 1 });
                        return;
                    }
                    cursor = node.right();
                }
            }
        }
        self.root = Self::new_node(DepthVal { val, prob: 1 });
    }

    pub fn get_random(&self) -> Option<T> {
        let mut cursor = self.root.as_ref().map(|n| n.clone());
        let mut rng = thread_rng();
        while let Some(node) = cursor {
            let left_depth = node.left().map_or_else(|| 0, |n| n.borrow().val.prob);
            let right_depth = node.right().map_or_else(|| 0, |n| n.borrow().val.prob);
            let choice = rng.gen_range(0..=left_depth + right_depth);
            if choice == 0 {
                return Some(node.borrow().val.val);
            }
            if choice <= left_depth {
                cursor = node.left()
            } else {
                cursor = node.right()
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    /// Since the test are rng dependent I am just asserting true and printing out results.
    /// Not sure what best practice is here
    #[test]
    fn random_tree_1() {
        let mut tree = BinaryTree::new() as RandomTree<u32>;
        for val in [4, 6, 2, 5, 3, 1, 7, 0, 8] {
            tree.insert_random(val);
        }
        let mut counts = [0; 9];
        for _ in 0..100_000 {
            let random = tree.get_random().expect("tree is not empty") as usize;
            counts[random] += 1;
        }
        println!("{:?}", counts);
    }

    /// 4 and 2 are expected to be twice as likely since they both appear twice in the tree
    #[test]
    fn random_tree_2() {
        let mut tree = BinaryTree::new() as RandomTree<u32>;
        for val in [4, 6, 2, 5, 3, 1, 7, 0, 8, 2, 4] {
            tree.insert_random(val);
        }
        let mut counts = [0; 9];
        for _ in 0..100_000 {
            let random = tree.get_random().expect("tree is not empty") as usize;
            counts[random] += 1;
        }
        println!("{:?}", counts);
    }

    #[test]
    fn random_tree_3() {
        let tree = BinaryTree::new() as RandomTree<u32>;
        assert_eq!(tree.get_random(), None);
    }
}
