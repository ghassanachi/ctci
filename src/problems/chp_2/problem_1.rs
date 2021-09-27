use crate::structures::{LinkedList, SinglyLinkedList};
use std::collections::HashSet;
use std::hash::Hash;

pub fn list_remove_duplicates<T: Eq + Copy + Hash>(list: &mut LinkedList<T>) {
    let mut duplicates: HashSet<T> = HashSet::new();
    for node in list.iter() {
        let val = &node.borrow().data.clone();
        if duplicates.contains(val) {
            list.unlink_node(node);
        }
        duplicates.insert(*val);
    }
}

pub fn singly_remove_duplicates<T: Eq + Copy + Hash>(list: &mut SinglyLinkedList<T>) {
    let mut duplicates: HashSet<T> = HashSet::new();
    let mut iter = list.iter();
    let mut current_node = match iter.next() {
        Some(node) => node,
        None => return,
    };

    let val = current_node.borrow().data;
    duplicates.insert(val);

    while let Some(next_node) = iter.next() {
        let next_val = next_node.borrow().data;
        if !duplicates.contains(&next_val) {
            duplicates.insert(next_val);
            current_node = next_node;
            continue;
        }
        // Skip next element in since it is already part of the iter state;
        current_node.borrow_mut().next = next_node.borrow_mut().next.take();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_remove_duplicates_1() {
        let test = vec![1, 2, 3, 4, 5, 2, 3, 4];
        let mut list = LinkedList::from_iter(test.into_iter());
        list_remove_duplicates(&mut list);
        let result: Vec<_> = list.values().collect();
        let expected: Vec<_> = (1..=5).into_iter().collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn list_remove_duplicates_2() {
        let test: Vec<i32> = vec![1, 2, 3];
        let mut list = LinkedList::from_iter(test);
        list_remove_duplicates(&mut list);
        let result: Vec<_> = list.values().collect();
        let expected: Vec<_> = (1..=3).into_iter().collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn list_remove_duplicates_3() {
        let test: Vec<i32> = vec![];
        let mut list = LinkedList::from_iter(test);
        list_remove_duplicates(&mut list);
        let result: Vec<_> = list.values().collect();
        assert_eq!(result, vec![]);
    }

    #[test]
    fn singly_remove_duplicates_1() {
        let test = vec![1, 2, 3, 4, 5, 2, 3, 4];
        let mut list = SinglyLinkedList::from_iter(test.into_iter());
        singly_remove_duplicates(&mut list);
        let result: Vec<_> = list.values().collect();
        let expected: Vec<_> = (1..=5).into_iter().collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn singly_remove_duplicates_2() {
        let test: Vec<i32> = vec![1, 2, 3];
        let mut list = SinglyLinkedList::from_iter(test);
        singly_remove_duplicates(&mut list);
        let result: Vec<_> = list.values().collect();
        let expected: Vec<_> = (1..=3).into_iter().collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn singly_remove_duplicates_3() {
        let test: Vec<i32> = vec![];
        let mut list = SinglyLinkedList::from_iter(test);
        singly_remove_duplicates(&mut list);
        let result: Vec<_> = list.values().collect();
        assert_eq!(result, vec![]);
    }
}
