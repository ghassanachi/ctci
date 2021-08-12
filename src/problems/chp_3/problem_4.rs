use std::fmt::Debug;

#[derive(Debug)]
pub struct StackQueue<T> {
    stack_enqueue: Vec<T>,
    stack_dequeue: Vec<T>,
}

impl<T> StackQueue<T> {
    pub fn new() -> Self {
        Self {
            stack_enqueue: Vec::new(),
            stack_dequeue: Vec::new(),
        }
    }

    pub fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        iter.into_iter().fold(Self::new(), |mut acc, val| {
            acc.enqueue(val);
            acc
        })
    }

    pub fn enqueue(&mut self, val: T) {
        self.stack_enqueue.push(val);
    }

    fn rebalance(&mut self) {
        if self.stack_dequeue.is_empty() {
            while let Some(val) = self.stack_enqueue.pop() {
                self.stack_dequeue.push(val)
            }
        }
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.rebalance();
        self.stack_dequeue.pop()
    }

    pub fn peek(&mut self) -> Option<&T> {
        self.rebalance();
        self.stack_dequeue.last()
    }

    pub fn len(&self) -> usize {
        self.stack_dequeue.len() + self.stack_enqueue.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn queue_stacks_1() {
        let mut stack = StackQueue::new();

        assert!(stack.is_empty());
        assert_eq!(stack.len(), 0);

        stack.enqueue(1);
        stack.enqueue(2);
        stack.enqueue(3);

        assert!(!stack.is_empty());
        assert_eq!(stack.len(), 3);

        assert_eq!(stack.peek(), Some(&1));
        assert_eq!(stack.dequeue(), Some(1));
        assert_eq!(stack.dequeue(), Some(2));
        assert_eq!(stack.dequeue(), Some(3));
        assert_eq!(stack.dequeue(), None);
    }

    #[test]
    fn queue_stacks_2() {
        let iter = vec![1, 2, 3, 4, 5];
        let mut stack = StackQueue::from_iter(iter);

        assert!(!stack.is_empty());
        assert_eq!(stack.len(), 5);

        assert_eq!(stack.peek(), Some(&1));
        assert_eq!(stack.dequeue(), Some(1));
        assert_eq!(stack.dequeue(), Some(2));
        assert_eq!(stack.dequeue(), Some(3));

        stack.enqueue(1);
        stack.enqueue(2);
        stack.enqueue(3);

        assert_eq!(stack.peek(), Some(&4));
        assert_eq!(stack.dequeue(), Some(4));
        assert_eq!(stack.dequeue(), Some(5));
        assert_eq!(stack.dequeue(), Some(1));
        assert_eq!(stack.dequeue(), Some(2));
        assert_eq!(stack.dequeue(), Some(3));
        assert_eq!(stack.dequeue(), None);
    }
}
