use std::cell::RefCell;
use std::convert::From;
use std::fmt;
use std::fmt::Display;
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;
use std::ptr;
use std::rc::Rc;

pub type NodeRef<T> = Rc<RefCell<Node<T>>>;

// Used specifically for hashing needs, like HashSet:
pub struct HashedNode<T>(NodeRef<T>);

/// # Copied from [modulitos/CtCI-rust][`ll-url`]
///
/// # Changes
/// - Removed some of the global bounds on T and added them where needed,
/// - Added [`values`] method which retruns an iterator on T (helps with testing)
/// - Added [`pop_front`] method which removes the first element of the linkedlist
/// - Added [`remove_next`] method on [Node], which makes is a little easier to write
/// functions operating directly on nodes and is quicker than traversing the while list.
///
/// [`values`]: #method.values
/// [`pop_front`]: #method.pop_front
/// [`ll-url`]: https://github.com/modulitos/CtCI-rust
/// [`remove_next`]: Node::remove_next
#[derive(Debug)]
pub struct LinkedList<T> {
    pub head: Option<NodeRef<T>>,
}

pub struct Node<T> {
    pub data: T,
    pub next: Option<NodeRef<T>>,
}

impl<T> Node<T> {
    /// Removes and returns the `next` element, and replaces with it's next element.
    ///
    /// # Warning
    /// There is currently a bug that might cause the iterator to return an element that has been
    /// removed. This would happen if the iterator currently holds a reference to it, and
    /// specifically if this element is the last in the list.
    // TODO: Fix above warning with a new Iter implementation which does not hold next;
    pub fn remove_next(&mut self) -> Option<NodeRef<T>>
    where
        T: Clone,
    {
        if let Some(next) = &mut self.next {
            let next_node = next.borrow_mut().next.take();
            if let Some(dst) = next_node {
                next.swap(&dst);
                return Some(Rc::clone(&dst));
            } else {
                self.next = None;
                return next_node;
            }
        }
        None
    }
}

impl<T: fmt::Debug> fmt::Debug for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Node {{ data: {:?}, next: {:?} }}", self.data, self.next)
    }
}

impl<T> LinkedList<T>
where
    T: std::clone::Clone,
{
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        <Self as FromIterator<T>>::from_iter(iter)
    }

    pub fn prepend(&mut self, new_value: T) {
        let new_node = Some(Rc::new(RefCell::new(Node {
            data: new_value,
            next: self.head.take(),
        })));
        self.head = new_node.clone();
    }

    /// Removes the first element of the LinkedaList
    pub fn pop_front(&mut self) -> Option<NodeRef<T>> {
        let mut next: Option<NodeRef<T>> = None;
        if let Some(ref head) = self.head {
            next = head.borrow_mut().next.take();
        }
        std::mem::replace(&mut self.head, next)
    }

    pub fn append(&mut self, new_value: T) {
        if let Some(tail) = self.tail() {
            tail.borrow_mut().next = Some(Rc::new(RefCell::new(Node {
                data: new_value,
                next: None,
            })));
        } else {
            self.head = Some(Rc::new(RefCell::new(Node {
                data: new_value,
                next: None,
            })));
        }
    }

    pub fn append_node(&mut self, node: NodeRef<T>) {
        if let Some(tail) = self.tail() {
            tail.borrow_mut().next = Some(node.clone());
        } else {
            self.head = Some(node.clone());
        }
    }

    // TODO: this isn't loop safe!
    // If we try and append to a LL that has a cycle, we'll be in an infinite loop.
    pub fn tail(&self) -> Option<NodeRef<T>> {
        for node in self.iter() {
            if let None = node.clone().borrow().next {
                return Some(node);
            }
        }
        None
    }

    /// Warning: this will not check that the provided node belongs to the current list.
    fn _unlink_node(&mut self, node_to_remove: NodeRef<T>) {
        for node in self.iter() {
            let mut borrowed_node = node.borrow_mut();
            if let Some(next) = borrowed_node.next.clone() {
                if Rc::ptr_eq(&next, &node_to_remove) {
                    borrowed_node.next = node_to_remove.borrow_mut().next.take();
                    break;
                }
            } else if Rc::ptr_eq(&node, &node_to_remove) {
                // handle case when node_to_remove is the only element
                // in the list
                self.head = None;
            }
        }
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.clone(),
        }
    }

    pub fn values(&self) -> ValuesIter<T> {
        ValuesIter { base: self.iter() }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(boxed_node) = cur_link {
            cur_link = boxed_node.borrow_mut().next.take()
        }
    }
}

impl<T> FromIterator<T> for LinkedList<T>
where
    T: Clone,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut c = LinkedList::new();
        for i in iter {
            c.append(i);
        }
        c
    }
}

#[derive(Debug)]
pub struct Iter<T> {
    next: Option<NodeRef<T>>,
}

impl<'a, T> Iterator for Iter<T> {
    type Item = NodeRef<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.next.clone() {
            // Set the new self.next:
            if let Some(new_next) = next.borrow().next.clone() {
                self.next = Some(new_next);
            } else {
                self.next = None;
            }
            return Some(next);
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct ValuesIter<T> {
    base: Iter<T>,
}

impl<'a, T: Clone> Iterator for ValuesIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.base.next().map(|node| {
            let val = node.borrow().data.clone();
            val
        })
    }
}

impl<T: Display> Display for LinkedList<T> {
    fn fmt(&self, w: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(w, "[")?;
        let mut node = self.head.clone();
        while let Some(n) = node {
            write!(w, "{}", n.borrow().data)?;
            node = n.borrow().next.clone();
            if node.is_some() {
                write!(w, ", ")?;
            }
        }
        write!(w, "]")
    }
}

impl<T: PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        if ptr::eq(self, other) {
            // For loop detection - if the nodes share the same
            // reference, they are equal.
            return true;
        }
        self.data == other.data && self.next == other.next
    }
}

// By implementing Eq, we are making the promise that our
// implementation of PartialEq is reflexive.
impl<T: Eq> Eq for Node<T> {}

impl<T: Hash> std::hash::Hash for HashedNode<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // TODO: make hash work for nodes that have the same data
        self.0.borrow().data.hash(state);
    }
}

impl<T> From<T> for HashedNode<T> {
    fn from(item: T) -> Self {
        HashedNode(Rc::new(RefCell::new(Node {
            data: item,
            next: None,
        })))
    }
}

impl<T> HashedNode<T> {
    pub fn from_node(node: NodeRef<T>) -> Self {
        HashedNode(node)
    }
}

impl<T: PartialEq> PartialEq for HashedNode<T> {
    fn eq(&self, other: &Self) -> bool {
        if ptr::eq(self, other) {
            // For loop detection - if the nodes share the same
            // reference, they are equal.
            return true;
        }
        let other_node = other.0.borrow();
        let self_node = self.0.borrow();
        self_node.data == other_node.data && self_node.next == other_node.next
    }
}

impl<T: Eq> Eq for HashedNode<T> {}

impl<T: PartialEq> PartialEq for LinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        self.head == other.head
    }

    fn ne(&self, other: &Self) -> bool {
        self.head != other.head
    }
}

impl<T: Eq + std::fmt::Debug> Eq for LinkedList<T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eq() {
        let mut list = LinkedList::new();
        list.append(1);
        list.append(2);
        list.append(3);
        let mut list2 = LinkedList::new();
        list2.append(1);
        list2.append(2);
        list2.append(3);

        assert_eq!(list, list2);
        list2 = LinkedList::new();
        list2.append(3);
        assert_ne!(list, list2);
        list = LinkedList::new();
        list.append(3);
        assert_eq!(list, list2);
    }

    #[test]
    fn prepend_and_append() {
        let mut list = LinkedList::new();
        list.prepend(2);
        list.prepend(1);
        list.append(3);
        let mut list2 = LinkedList::new();
        list2.append(1);
        list2.append(2);
        list2.append(3);

        assert_eq!(list, list2);
        list2.prepend(1);
        assert_ne!(list, list2);
        list.prepend(1);
        assert_eq!(list, list2);
    }

    #[test]
    fn eq_append_node() {
        let shared_node = Rc::new(RefCell::new(Node {
            data: 1,
            next: None,
        }));
        let mut list1 = LinkedList::new();
        list1.append_node(shared_node.clone());

        let mut list2 = LinkedList::new();
        list2.append(1);

        assert_eq!(list1, list2);

        let mut list3 = LinkedList::new();
        list3.append_node(shared_node);
        list3.append(2);

        let mut list4 = LinkedList::new();
        list4.append(1);
        list4.append(2);

        assert_eq!(list3, list4);

        // These are no longer equal! Because a new node has been added to the shared_node:
        assert_ne!(list1, list2);
        let mut list2 = LinkedList::new();
        list2.append(1);
        list2.append(2);

        assert_eq!(list1, list2);
    }

    #[test]
    fn append_many_nodes() {
        let node = Rc::new(RefCell::new(Node {
            data: 3,
            next: None,
        }));
        let mut list1 = LinkedList::new();
        list1.append(1);
        list1.append(2);
        list1.append(3);

        let mut list2 = LinkedList::new();
        list2.append(1);
        list2.append(2);
        list2.append_node(node);

        assert_eq!(list1, list2);
        list1.append(4);
        assert_ne!(list1, list2);
        list2.append(4);
        assert_eq!(list1, list2);
        let node = Rc::new(RefCell::new(Node {
            data: 9,
            next: None,
        }));
        list1.append_node(node.clone());
        list2.append_node(node);
        list2.append(3);
        assert_eq!(list1, list2);
    }

    #[test]
    fn eq_with_cycle() {
        let first_node = Rc::new(RefCell::new(Node {
            data: 1,
            next: None,
        }));
        let mut list = LinkedList::new();
        list.append_node(first_node.clone());
        list.append(2);
        list.append(3);
        list.append_node(first_node.clone());

        let mut list2 = LinkedList::new();
        list2.append(1);

        assert_ne!(list, list2);

        let mut list2 = LinkedList::new();
        list2.append_node(first_node);

        assert_eq!(list, list2);
    }

    #[test]
    // Tests that our nodes can be hashed and saved into a set.
    fn hashset_iter_nodes() {
        let node = Rc::new(RefCell::new(Node {
            data: 9,
            next: None,
        }));
        let mut list = LinkedList::new();
        list.append(1);
        list.append(2);
        list.append(3);
        list.append_node(node.clone());

        let mut set = std::collections::HashSet::new();
        // iterate over nodes, adding each node to our hashset:
        for node in list.iter() {
            set.insert(HashedNode::from_node(node));
        }
        assert_eq!(set.contains(&HashedNode::from_node(node)), true);
        assert_eq!(set.contains(&HashedNode::from(4)), false);
        assert_eq!(set.len(), 4);
    }
}
