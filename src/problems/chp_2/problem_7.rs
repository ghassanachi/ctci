use crate::linkedlist::*;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;

pub fn list_intersection_with_set<T: Clone + Eq + Hash + Debug>(
    l1: &SinglyLinkedList<T>,
    l2: &SinglyLinkedList<T>,
) -> bool {
    // Create a Node set using the RefCell raw pointer to reduce memory usage
    let l1_nodes: HashSet<*mut Node<T>> = l1.iter().map(|node| node.as_ptr()).collect();

    for node in l2.iter() {
        if l1_nodes.contains(&node.as_ptr()) {
            return true;
        }
    }
    return false;
}

// If there is an intersection then tails will be the same
pub fn list_intersection_with_tail<T: Clone + Eq + Hash>(
    l1: &SinglyLinkedList<T>,
    l2: &SinglyLinkedList<T>,
) -> bool {
    if let (Some(l1_tail), Some(l2_tail)) = (l1.tail(), l2.tail()) {
        return Rc::ptr_eq(&l1_tail, &l2_tail);
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_intersection_1() {
        let l1 = vec![7, 1, 6];
        let l1 = SinglyLinkedList::from_iter(l1);
        let l2 = vec![5, 9, 2];
        let l2 = SinglyLinkedList::from_iter(l2);

        assert!(!list_intersection_with_set(&l1, &l2));
        assert!(!list_intersection_with_tail(&l1, &l2));
    }

    #[test]
    fn list_intersection_2() {
        let l1 = vec![7, 1, 6];
        let mut l1 = SinglyLinkedList::from_iter(l1);
        let l2 = vec![5, 9, 2];
        let mut l2 = SinglyLinkedList::from_iter(l2);

        // Append new shared tail to l1 and l2
        let tail_list = vec![1, 2, 3];
        let tail_list = SinglyLinkedList::from_iter(tail_list);
        l1.append_node(Rc::clone(tail_list.head.as_ref().unwrap()));
        l2.append_node(Rc::clone(tail_list.head.as_ref().unwrap()));

        assert!(list_intersection_with_set(&l1, &l2));
        assert!(list_intersection_with_tail(&l1, &l2));
    }

    #[test]
    fn list_intersection_3() {
        let l1 = vec![];
        let mut l1 = SinglyLinkedList::from_iter(l1);
        let l2 = vec![];
        let mut l2 = SinglyLinkedList::from_iter(l2);

        assert!(!list_intersection_with_set(&l1, &l2));
        assert!(!list_intersection_with_tail(&l1, &l2));

        let tail_list = vec![1, 2, 3];
        let tail_list = SinglyLinkedList::from_iter(tail_list);
        l1.append_node(Rc::clone(tail_list.head.as_ref().unwrap()));
        l2.append_node(Rc::clone(tail_list.head.as_ref().unwrap()));

        assert!(list_intersection_with_set(&l1, &l2));
        assert!(list_intersection_with_tail(&l1, &l2));
    }
}
