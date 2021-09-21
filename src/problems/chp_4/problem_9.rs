use crate::structures::TreeNodeRef;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

pub fn weave<T: Copy + Debug>(left: &[T], right: &[T]) -> Vec<Vec<T>> {
    let mut result = Vec::new();
    let mut temp = Vec::new();
    weave_helper(
        &mut Vec::from(left.clone()),
        &mut Vec::from(right.clone()),
        &mut temp,
        &mut result,
    );
    return result;
}

fn weave_helper<T: Copy + Debug>(
    left: &mut Vec<T>,
    right: &mut Vec<T>,
    temp: &mut Vec<T>,
    results: &mut Vec<Vec<T>>,
) {
    if left.is_empty() || right.is_empty() {
        let mut result = temp.clone();
        result.append(&mut left.clone());
        result.append(&mut right.clone());
        results.push(result);
        return;
    }

    let first_left = left.remove(0);
    temp.push(first_left);
    weave_helper(left, right, temp, results);
    temp.pop();
    left.insert(0, first_left);

    let first_right = right.remove(0);
    temp.push(first_right);
    weave_helper(left, right, temp, results);
    temp.pop();
    right.insert(0, first_right);
}

fn bst_sequences_helper<T: Copy + Debug + Eq + Hash>(root: TreeNodeRef<T>) -> Vec<Vec<T>> {
    let mut result: Vec<Vec<T>> = Vec::new();
    if let Some(root) = root {
        let cur_val = root.borrow().val;

        let left_node = root.borrow().left.as_ref().map(|n| n.clone());
        let right_node = root.borrow().right.as_ref().map(|n| n.clone());

        let left_sequences = bst_sequences(left_node);
        let right_sequences = bst_sequences(right_node);

        for left in &left_sequences {
            for right in &right_sequences {
                let weaved_sequences = weave(left, right);
                for mut weave in weaved_sequences {
                    weave.insert(0, cur_val);
                    result.push(weave);
                }
            }
        }
        return result;
    }
    result.push(Vec::new());
    result
}

pub fn bst_sequences<T: Copy + Debug + Eq + Hash>(root: TreeNodeRef<T>) -> Vec<Vec<T>> {
    bst_sequences_helper(root)
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::structures::BinaryTree;

    #[test]
    fn bst_weave_1() {
        let result = weave(&vec![1, 2, 3], &vec![4, 5, 6]);
        assert_eq!(result.len(), 20);
    }

    #[test]
    fn bst_sequences_1() {
        let tree = BinaryTree::build(&vec![Some(2), Some(3), None, None, Some(4)]);
        let result: Vec<Vec<_>> = bst_sequences(tree.root).into_iter().collect();
        assert_eq!(result.len(), 2)
    }

    #[test]
    fn bst_sequences_2() {
        let tree = BinaryTree::<u32>::build(&vec![]);
        let result: Vec<Vec<_>> = bst_sequences(tree.root).into_iter().collect();
        assert_eq!(result.len(), 1);
        assert_eq!(result, vec![vec![]]);
    }

    #[test]
    fn bst_sequences_3() {
        let tree = BinaryTree::build(&vec![Some(2), Some(3), None, Some(12), None, None, Some(4)]);
        tree.print();
        let result: Vec<Vec<_>> = bst_sequences(tree.root).into_iter().collect();
        assert_eq!(result.len(), 3)
    }

    #[test]
    fn bst_sequences_4() {
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
            None,
            None,
        ]);
        tree.print();
        let result: Vec<Vec<_>> = bst_sequences(tree.root).into_iter().collect();
        let unique: HashSet<Vec<_>> = result.clone().into_iter().collect();
        assert_eq!(result.len(), 80);
        assert_eq!(result.len(), unique.len());
    }
}
