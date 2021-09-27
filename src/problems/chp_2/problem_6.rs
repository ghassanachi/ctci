use crate::structures::{NodeRef, SinglyLinkedList};
use std::rc::Rc;

pub fn reverse<T>(head: &mut Option<NodeRef<T>>) -> Option<NodeRef<T>> {
    let mut prev: Option<NodeRef<T>> = None;
    let mut current_node = head.take();
    while let Some(current_node_inner) = current_node.take() {
        let temp = { Rc::clone(&current_node_inner).borrow_mut().next.take() };
        current_node_inner.borrow_mut().next = prev.take();
        prev = Some(current_node_inner);
        current_node = temp;
    }
    prev
}

pub fn list_palindrome(list: &mut SinglyLinkedList<char>) -> bool {
    let inner_list = &*list;

    let mut slow_iter = inner_list.iter();
    let mut fast_iter = inner_list.iter();
    let mut slow: Option<NodeRef<char>> = None;

    loop {
        match (fast_iter.next(), slow_iter.next()) {
            (Some(_), Some(slow_inner)) => {
                fast_iter.nth(0);
                slow = Some(Rc::clone(&slow_inner))
            }
            _ => break,
        }
    }

    // take the next element after slow (handles odd length cases)
    if let Some(slow_inner) = slow {
        slow = slow_inner.borrow_mut().next.take()
    }

    // Create list starting at slow pointer and reverse it.
    // tail_list will have equal or 1 less node than list
    let mut tail_list = SinglyLinkedList::new();
    tail_list.head = slow;
    tail_list.head = reverse(&mut tail_list.head);

    let mut slow_iter = tail_list.values();
    let mut front_iter = inner_list.values();

    // we don't need to handle the (None, Some(_)) case since it will only occur one and
    // will always be valid if it makes it that far
    loop {
        match (slow_iter.next(), front_iter.next()) {
            (Some(slow_val), Some(front_val)) => {
                if slow_val != front_val {
                    return false;
                }
            }
            _ => break,
        }
    }

    // recreate original list by reversing tail and adding it back to list
    tail_list.head = reverse(&mut tail_list.head);

    if let Some(last) = list.tail() {
        last.borrow_mut().next = tail_list.head.take()
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_palindrome_1() {
        let l1 = String::from("teet");

        let mut l1 = SinglyLinkedList::from_iter(l1.chars());
        let is_palindrome = list_palindrome(&mut l1);
        assert!(is_palindrome);

        l1.append('h');
        let is_palindrome = list_palindrome(&mut l1);
        assert!(!is_palindrome);
    }

    #[test]
    fn list_palindrome_2() {
        let l1 = String::from("racecar");

        let mut l1 = SinglyLinkedList::from_iter(l1.chars());
        let is_palindrome = list_palindrome(&mut l1);
        assert!(is_palindrome);

        l1.append('s');
        let is_palindrome = list_palindrome(&mut l1);
        assert!(!is_palindrome);
    }

    #[test]
    fn list_palindrome_3() {
        let l1 = String::from("");

        let mut l1 = SinglyLinkedList::from_iter(l1.chars());
        let is_palindrome = list_palindrome(&mut l1);

        assert!(is_palindrome);

        l1.append('s');
        let is_palindrome = list_palindrome(&mut l1);
        assert!(is_palindrome);

        l1.append('s');
        let is_palindrome = list_palindrome(&mut l1);
        assert!(is_palindrome);
    }
}
