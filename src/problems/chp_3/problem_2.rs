use std::fmt::Debug;
use std::iter::FromIterator;

#[derive(Debug)]
pub struct MinStack<T> {
    stack: Vec<T>,
    min_so_far: Vec<T>,
}

impl<T> MinStack<T>
where
    T: Eq + PartialEq + PartialOrd + Ord + Debug + Copy,
{
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            min_so_far: Vec::new(),
        }
    }

    pub fn push(&mut self, val: T) {
        match self.min_so_far.last() {
            Some(min) if *min < val => {}
            _ => self.min_so_far.push(val),
        }
        self.stack.push(val);
    }

    pub fn pop(&mut self) -> Option<T> {
        let out = self.stack.pop()?;
        // Safe because if out is not empty nor will min
        let min = self.min_so_far.last().unwrap();
        if out == *min {
            self.min_so_far.pop();
        }
        Some(out)
    }

    pub fn min(&self) -> Option<T> {
        self.min_so_far.last().map(|min| *min)
    }
}

impl<T> FromIterator<T> for MinStack<T>
where
    T: Eq + PartialEq + PartialOrd + Ord + Debug + Copy,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        iter.into_iter().fold(Self::new(), |mut acc, val| {
            acc.push(val);
            acc
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_stack_1() {
        let iter = vec![5, 2, 4, 12, 10, 6, 3, 1];
        let mut min_stack = MinStack::from_iter(iter);

        assert_eq!(min_stack.min(), Some(1));
        assert_eq!(min_stack.pop(), Some(1));
        assert_eq!(min_stack.pop(), Some(3));
        assert_eq!(min_stack.min(), Some(2));

        assert_eq!(min_stack.pop(), Some(6));
        assert_eq!(min_stack.pop(), Some(10));
        assert_eq!(min_stack.pop(), Some(12));

        assert_eq!(min_stack.min(), Some(2));

        assert_eq!(min_stack.pop(), Some(4));
        assert_eq!(min_stack.pop(), Some(2));
        assert_eq!(min_stack.min(), Some(5));
        assert_eq!(min_stack.pop(), Some(5));

        assert_eq!(min_stack.min(), None);
        assert_eq!(min_stack.pop(), None);
    }

    #[test]
    fn min_stack_2() {
        let mut min_stack = MinStack::new();

        assert_eq!(min_stack.min(), None);
        assert_eq!(min_stack.pop(), None);

        min_stack.push(1);
        min_stack.push(3);
        min_stack.push(2);

        assert_eq!(min_stack.min(), Some(1));
        assert_eq!(min_stack.pop(), Some(2));
        assert_eq!(min_stack.pop(), Some(3));
        assert_eq!(min_stack.min(), Some(1));

        assert_eq!(min_stack.pop(), Some(1));

        assert_eq!(min_stack.min(), None);
        assert_eq!(min_stack.pop(), None);
    }

    #[test]
    fn min_stack_3() {
        let mut min_stack = MinStack::new();

        assert_eq!(min_stack.min(), None);
        assert_eq!(min_stack.pop(), None);

        min_stack.push(1);
        min_stack.push(1);
        min_stack.push(1);
        min_stack.push(1);

        assert_eq!(min_stack.min(), Some(1));
        assert_eq!(min_stack.pop(), Some(1));
        assert_eq!(min_stack.min(), Some(1));
        assert_eq!(min_stack.min(), Some(1));
        assert_eq!(min_stack.min(), Some(1));
        assert_eq!(min_stack.pop(), Some(1));
        assert_eq!(min_stack.pop(), Some(1));
        assert_eq!(min_stack.pop(), Some(1));

        assert_eq!(min_stack.min(), None);
        assert_eq!(min_stack.pop(), None);
    }
}
