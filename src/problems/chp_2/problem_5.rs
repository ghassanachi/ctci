use crate::structures::*;
use std::cell::RefCell;
use std::rc::Rc;

pub fn sum_lists_backwards(
    l1: &SinglyLinkedList<i32>,
    l2: &SinglyLinkedList<i32>,
) -> SinglyLinkedList<i32> {
    let mut res = SinglyLinkedList::new();
    let mut l1 = l1.values();
    let mut l2 = l2.values();

    let mut remainder = 0;
    loop {
        match (l1.next(), l2.next()) {
            (None, Some(val)) | (Some(val), None) => {
                let val = val + remainder;
                res.append(val % 10);
                remainder = val / 10;
            }
            (Some(l1_val), Some(l2_val)) => {
                let val = l1_val + l2_val + remainder;
                res.append(val % 10);
                remainder = val / 10;
            }
            _ => break,
        }
    }
    if remainder != 0 {
        res.append(remainder);
    }
    res
}

fn _recurse(l1: &Option<NodeRef<i32>>, l2: &Option<NodeRef<i32>>) -> Option<(i32, NodeRef<i32>)> {
    match (l1, l2) {
        (Some(l1_node), Some(l2_node)) => {
            let val = l1_node.borrow().data + l2_node.borrow().data;
            let l1_node = Rc::clone(l1_node);
            let l2_node = Rc::clone(l2_node);
            let mut remainder = 0;
            let mut next: Option<NodeRef<i32>> = None;
            if let Some((r, n)) = _recurse(&l1_node.borrow().next, &l2_node.borrow().next) {
                remainder = r;
                next = Some(n);
            }
            let val = val + remainder;
            let node = Rc::new(RefCell::new(Node {
                data: val % 10,
                next,
            }));
            return Some((val / 10, node));
        }
        (None, None) => return None,
        _ => panic!("list should have equal length"),
    };
}

pub fn sum_lists_forward(
    l1: &mut SinglyLinkedList<i32>,
    l2: &mut SinglyLinkedList<i32>,
) -> SinglyLinkedList<i32> {
    let mut res = SinglyLinkedList::new();
    let l1_len = l1.iter().count();
    let l2_len = l2.iter().count();
    if l1_len < l2_len {
        for _ in l1_len..l2_len {
            l1.prepend(0);
        }
    } else {
        for _ in l2_len..l1_len {
            l2.prepend(0);
        }
    }

    if let Some((remainder, head)) = _recurse(&l1.head, &l2.head) {
        res.head = Some(head);
        if remainder != 0 {
            res.prepend(remainder);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_lists_back_1() {
        let l1 = vec![7, 1, 6];
        let l2 = vec![5, 9, 2];

        let l1 = SinglyLinkedList::from_iter(l1);
        let l2 = SinglyLinkedList::from_iter(l2);

        let result = sum_lists_backwards(&l1, &l2);
        let result: Vec<_> = result.values().collect();
        assert_eq!(result, vec![2, 1, 9]);
    }

    #[test]
    fn sum_lists_back_2() {
        let l1 = vec![9, 7, 8];
        let l2 = vec![6, 8, 5];

        let l1 = SinglyLinkedList::from_iter(l1);
        let l2 = SinglyLinkedList::from_iter(l2);

        let result = sum_lists_backwards(&l1, &l2);
        let result: Vec<_> = result.values().collect();
        assert_eq!(result, vec![5, 6, 4, 1]);
    }

    #[test]
    fn sum_lists_back_3() {
        let l1 = vec![8, 3];
        let l2 = vec![6, 7, 9, 1];

        let l1 = SinglyLinkedList::from_iter(l1);
        let l2 = SinglyLinkedList::from_iter(l2);

        let result = sum_lists_backwards(&l1, &l2);
        let result: Vec<_> = result.values().collect();
        assert_eq!(result, vec![4, 1, 0, 2]);
    }

    #[test]
    fn sum_lists_forward_1() {
        let l1 = vec![7, 1, 6];
        let l2 = vec![5, 9, 2];

        let mut l1 = SinglyLinkedList::from_iter(l1);
        let mut l2 = SinglyLinkedList::from_iter(l2);

        let result = sum_lists_forward(&mut l1, &mut l2);
        let result: Vec<_> = result.values().collect();
        assert_eq!(result, vec![1, 3, 0, 8]);
    }

    #[test]
    fn sum_lists_forward_2() {
        let l1 = vec![9, 7, 8, 1, 3];
        let l2 = vec![6, 8];

        let mut l1 = SinglyLinkedList::from_iter(l1);
        let mut l2 = SinglyLinkedList::from_iter(l2);

        let result = sum_lists_forward(&mut l1, &mut l2);
        let result: Vec<_> = result.values().collect();
        assert_eq!(result, vec![9, 7, 8, 8, 1]);
    }

    #[test]
    fn sum_lists_forward_3() {
        let l1 = vec![8, 3];
        let l2 = vec![6, 7, 9, 1];

        let mut l1 = SinglyLinkedList::from_iter(l1);
        let mut l2 = SinglyLinkedList::from_iter(l2);

        let result = sum_lists_forward(&mut l1, &mut l2);
        let result: Vec<_> = result.values().collect();
        assert_eq!(result, vec![6, 8, 7, 4]);
    }
}
