use crate::structures::{NodeRef, SinglyLinkedList};
use std::fmt::Debug;
use std::rc::Rc;

pub fn loop_detection<T: Clone + Debug>(l1: &SinglyLinkedList<T>) -> Option<NodeRef<T>> {
    let mut fast_iter = l1.iter();
    let mut slow_iter = l1.iter();
    // 1. Check for cycle
    loop {
        fast_iter.next();
        if let (Some(front), Some(back)) = (fast_iter.next(), slow_iter.next()) {
            if Rc::ptr_eq(&front, &back) {
                break;
            }
        } else {
            return None;
        }
    }

    // let dist(start, loop_start) = k
    // Collision point in loop will happen at loop_start_node - k
    // So we can reset slow_iter to the start and move both iterators at the same speed to find the
    // loop_start_node
    let mut slow_iter = l1.iter();
    loop {
        if let (Some(front), Some(back)) = (fast_iter.next(), slow_iter.next()) {
            if Rc::ptr_eq(&front, &back) {
                return Some(Rc::clone(&front));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loop_detection_1() {
        let l1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let l1 = SinglyLinkedList::from_iter(l1);

        let middle = l1.iter().nth(4).unwrap();
        let tail = l1.tail().unwrap();
        tail.borrow_mut().next = Some(Rc::clone(&middle));

        let loop_start_node = loop_detection(&l1).unwrap();
        assert!(Rc::ptr_eq(&loop_start_node, &middle));
    }

    #[test]
    fn loop_detection_2() {
        let l1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let l1 = SinglyLinkedList::from_iter(l1);

        assert_eq!(None, loop_detection(&l1));
    }

    #[test]
    fn loop_detection_3() {
        let l1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let l1 = SinglyLinkedList::from_iter(l1);

        let middle = l1.iter().next().unwrap();
        let tail = l1.tail().unwrap();
        tail.borrow_mut().next = Some(Rc::clone(&middle));

        let loop_start_node = loop_detection(&l1).unwrap();
        assert!(Rc::ptr_eq(&loop_start_node, &middle));
    }
}
