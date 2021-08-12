use std::fmt::Debug;

#[derive(Debug)]
pub struct Stack<T>(Vec<T>);

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut rev: Vec<T> = iter.into_iter().collect();
        rev.reverse();
        Self(rev)
    }

    pub fn push(&mut self, val: T) {
        self.0.push(val)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.0.last()
    }

    pub fn is_empty(&self) -> bool {
        self.0.len() == 0
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            inner: self.0.iter(),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            inner: self.0.iter_mut(),
        }
    }
}

pub struct Iter<'a, T> {
    inner: std::slice::Iter<'a, T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next_back()
    }
}

pub struct IterMut<'a, T> {
    inner: std::slice::IterMut<'a, T>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next_back()
    }
}

impl<T> IntoIterator for Stack<T> {
    type Item = T;
    type IntoIter = std::iter::Rev<std::vec::IntoIter<T>>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter().rev()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn basic() {
        let mut stack = Stack::new();
        assert!(stack.is_empty());
        assert_eq!(stack.peek(), None);
        assert_eq!(stack.pop(), None);

        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert!(!stack.is_empty());
        assert_eq!(stack.peek(), Some(&3));
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);

        assert!(stack.is_empty());
        assert_eq!(stack.peek(), None);
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn from_into_iter() {
        let iter = vec![1, 2, 3, 4, 5];
        let mut stack = Stack::from_iter(iter);

        assert!(!stack.is_empty());
        assert_eq!(stack.peek(), Some(&1));
        assert_eq!(stack.pop(), Some(1));

        let result: Vec<_> = stack.into_iter().collect();
        let expected: Vec<_> = (2..=5).collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn iter() {
        let iter = vec![1, 2, 3, 4, 5];
        let mut stack = Stack::from_iter(iter);

        assert!(!stack.is_empty());
        assert_eq!(stack.peek(), Some(&1));
        assert_eq!(stack.pop(), Some(1));

        let result: Vec<_> = stack.iter().collect();
        assert_eq!(result, vec![&2, &3, &4, &5]);
    }

    #[test]
    fn iter_mut() {
        let iter = vec![1, 2, 3, 4, 5];
        let mut stack = Stack::from_iter(iter);

        assert!(!stack.is_empty());
        assert_eq!(stack.peek(), Some(&1));
        assert_eq!(stack.pop(), Some(1));

        let result: Vec<_> = stack.iter().collect();
        assert_eq!(result, vec![&mut 2, &mut 3, &mut 4, &mut 5]);
    }
}
