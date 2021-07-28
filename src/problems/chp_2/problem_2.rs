use crate::linkedlist::*;
use std::rc::Rc;

pub fn find_kth_last_element<T: Clone>(list: &SinglyLinkedList<T>, k: usize) -> Option<T> {
    let mut front = list.iter();
    let mut back = list.iter();
    for _ in 0..k {
        if front.next().is_none() {
            return None;
        }
    }

    let mut last_node: Option<NodeRef<T>> = None;
    loop {
        match (back.next(), front.next()) {
            (Some(node), None) => return Some(node.as_ref().borrow().data.clone()),
            (Some(node), Some(_)) => last_node = Some(Rc::clone(&node)),
            (None, None) => return last_node.map(|node| node.as_ref().borrow().data.clone()),
            // If the list contains a cycle it will run forever
            // It is impossible for the back iterator finish first
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_kth_last_element_1() {
        let test = vec![1, 3, 4, 5, 2, 3, 4];
        let list = SinglyLinkedList::from_iter(test.into_iter());
        let result = find_kth_last_element(&list, 3);
        assert_eq!(result, Some(2));
    }

    #[test]
    fn find_kth_last_element_2() {
        let test = vec![1, 2, 3, 5, 2, 3, 4];
        let list = SinglyLinkedList::from_iter(test.into_iter());
        let result = find_kth_last_element(&list, 0);
        assert_eq!(result, Some(4));
    }

    #[test]
    fn find_kth_last_element_3() {
        let test = vec![2, 3, 4];
        let list = SinglyLinkedList::from_iter(test);
        let result = find_kth_last_element(&list, 4);
        assert_eq!(result, None);
    }
}
