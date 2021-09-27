use crate::structures::{Node, SinglyLinkedList};
use std::cell::RefCell;
use std::fmt::{Debug, Display};
use std::mem;
use std::rc::Rc;

pub fn get_next_data<T: Clone>(node: &Rc<RefCell<Node<T>>>) -> Option<T> {
    let node = Rc::clone(&node);
    let next = &node.borrow().next;
    next.as_deref().map(|node| node.borrow().data.clone())
}

pub fn partition<T: Clone + Default + Ord + PartialOrd + Debug + Display>(
    list: &mut SinglyLinkedList<T>,
    partition_on: T,
) {
    list.prepend(T::default());
    let mut iter = list.iter();

    let mut tail_list = SinglyLinkedList::<T>::new();
    tail_list.prepend(T::default());
    let mut tail = Rc::clone(&tail_list.head.as_ref().unwrap());

    while let Some(node) = iter.next() {
        while let Some(data) = get_next_data(&node) {
            if data < partition_on {
                break;
            }
            let next_node = { node.borrow_mut().remove_next() };
            if let Some(n) = next_node {
                tail.borrow_mut().next = Some(Rc::clone(&n));
                tail = n;
            }
        }
    }

    // Add tail to list
    let mut iter = list.iter();
    while let Some(node) = iter.next() {
        let end = node.borrow().next.is_none();
        if end {
            if let Some(tail) = &tail_list.head {
                node.borrow_mut().next = tail.borrow_mut().next.take()
            }
        }
    }
    let mut head = { list.head.take().unwrap().borrow_mut().next.take() };
    mem::swap(&mut list.head, &mut head);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn verify_partition<T: Clone + Ord + PartialOrd>(list: &SinglyLinkedList<T>, value: T) -> bool {
        let mut is_above = false;
        for node in list.iter() {
            let curr_value = node.borrow().data.clone();
            if curr_value < value && is_above {
                return false;
            }
            if curr_value >= value {
                is_above = true;
            }
        }
        true
    }

    #[test]
    fn partition_1() {
        let original = vec![9, 2, 3, 5, 1, 2, 0, 6, 2];
        let partition_on = 5;

        let mut list = SinglyLinkedList::from_iter(original);

        partition(&mut list, partition_on);

        assert!(verify_partition(&list, partition_on));
    }

    #[test]
    fn partition_2() {
        let original = vec![9, 2, 3, 5, 1, 2, 0, 6, 2];
        let partition_on = 2;

        let mut list = SinglyLinkedList::from_iter(original);
        partition(&mut list, partition_on);

        assert!(verify_partition(&list, partition_on));
    }

    #[test]
    fn partition_3() {
        let original = vec![];
        let partition_on = 2;

        let mut list = SinglyLinkedList::from_iter(original);

        partition(&mut list, partition_on);

        assert!(verify_partition(&list, partition_on));
    }
}
