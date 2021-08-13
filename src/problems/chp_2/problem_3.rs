use crate::structures::*;
use std::fmt::Debug;
use std::rc::Rc;

pub fn delete_middle_node<T: Clone + Default + Debug>(node: Option<NodeRef<T>>) {
    let mut cursor = node;
    while let Some(ref current_node) = cursor {
        let next = if let Some(next) = &current_node.borrow().next {
            Some(Rc::clone(next))
        } else {
            None
        };
        if let Some(node) = next {
            // Check if we are at the end
            let end = { node.borrow().next.is_none() };
            // Clone the data
            current_node.borrow_mut().data = node.borrow().data.clone();
            // Remove next child
            if end {
                current_node.borrow_mut().remove_next();
                cursor.take();
            } else {
                cursor = Some(node);
            }
        } else {
            // Node has no next
            cursor.take();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delete_middle_node_1() {
        let original = (0..=10).into_iter();
        let mut expected: Vec<_> = original.clone().collect();

        let list = SinglyLinkedList::from_iter(original);

        let node = list.iter().nth(2);
        expected.remove(2);

        delete_middle_node(node);

        let result: Vec<_> = list.values().collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn delete_middle_node_2() {
        let original = (0..5).zip(0..5).fold(Vec::new(), |mut acc, el| {
            acc.push(el.0);
            acc.push(el.1);
            acc
        });
        let mut expected = original.clone();

        let list = SinglyLinkedList::from_iter(original.into_iter());

        let node = list.iter().nth(6);
        expected.remove(6);

        delete_middle_node(node);

        let result: Vec<_> = list.values().collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn delete_middle_node_3() {
        // Removing last node from list is impossible.
        let original = (0..5).zip(0..5).fold(Vec::new(), |mut acc, el| {
            acc.push(el.0);
            acc.push(el.1);
            acc
        });
        let expected = original.clone();

        let list = SinglyLinkedList::from_iter(original.into_iter());

        let node = list.iter().nth(expected.len());

        delete_middle_node(node);

        let result: Vec<_> = list.values().collect();
        assert_eq!(result, expected);
    }
}
