use std::fmt::Debug;

#[derive(Debug)]
pub struct SetOfStacks<T> {
    stacks: Vec<Vec<T>>,
    capacity: usize,
}

impl<T> SetOfStacks<T> {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity != 0, "capacity cannot be 0");
        Self {
            stacks: Vec::new(),
            capacity,
        }
    }

    pub fn from_iter<I: IntoIterator<Item = T>>(iter: I, capacity: usize) -> Self {
        iter.into_iter().fold(Self::new(capacity), |mut acc, val| {
            acc.push(val);
            acc
        })
    }

    fn get_push_stack<'a>(&'a mut self) -> &'a mut Vec<T> {
        match self.stacks.last() {
            Some(last) if last.len() != last.capacity() => {}
            _ => {
                self.stacks.push(Vec::with_capacity(self.capacity));
            }
        };
        self.stacks.last_mut().unwrap()
    }

    fn get_pop_stack(&mut self) -> Option<&mut Vec<T>> {
        self.stacks.last_mut()
    }

    pub fn push(&mut self, val: T) {
        self.get_push_stack().push(val);
    }

    pub fn pop(&mut self) -> Option<T> {
        let stack = self.get_pop_stack()?;
        let out = stack.pop();
        // if empty remove last
        if stack.len() == 0 {
            self.stacks.pop();
        }
        out
    }

    pub fn pop_at(&mut self, stack_idx: usize) -> Option<T> {
        let stack = self.stacks.get_mut(stack_idx)?;
        let out = stack.pop();
        if stack.len() == 0 {
            self.stacks.remove(stack_idx);
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_of_stacks_1() {
        let iter = vec![5, 2, 4, 12, 10, 6, 3, 1, 14];
        let mut set_of_stacks = SetOfStacks::from_iter(iter, 3);

        assert_eq!(set_of_stacks.pop_at(0), Some(4));
        assert_eq!(set_of_stacks.pop(), Some(14));
        assert_eq!(set_of_stacks.pop_at(1), Some(6));
        assert_eq!(set_of_stacks.pop(), Some(1));
        assert_eq!(set_of_stacks.pop_at(2), Some(3));
        assert_eq!(set_of_stacks.pop(), Some(10));
        assert_eq!(set_of_stacks.pop(), Some(12));

        set_of_stacks.push(3);
        set_of_stacks.push(2);
        set_of_stacks.push(1);

        assert_eq!(set_of_stacks.pop(), Some(1));
        assert_eq!(set_of_stacks.pop(), Some(2));
        assert_eq!(set_of_stacks.pop(), Some(3));

        assert_eq!(set_of_stacks.pop(), Some(2));
        assert_eq!(set_of_stacks.pop(), Some(5));

        assert_eq!(set_of_stacks.pop(), None);
    }

    #[test]
    fn set_of_stacks_2() {
        let mut set_of_stacks = SetOfStacks::new(1);

        assert_eq!(set_of_stacks.pop(), None);

        set_of_stacks.push(1);
        set_of_stacks.push(3);
        set_of_stacks.push(2);

        assert_eq!(set_of_stacks.pop(), Some(2));
        assert_eq!(set_of_stacks.pop(), Some(3));

        set_of_stacks.push(3);
        set_of_stacks.push(2);
        set_of_stacks.push(1);

        assert_eq!(set_of_stacks.pop_at(3), Some(1));
        assert_eq!(set_of_stacks.pop_at(2), Some(2));
        assert_eq!(set_of_stacks.pop_at(1), Some(3));
        assert_eq!(set_of_stacks.pop(), Some(1));

        assert_eq!(set_of_stacks.pop(), None);
    }

    #[test]
    fn set_of_stacks_3() {
        let mut set_of_stacks = SetOfStacks::new(3);

        assert_eq!(set_of_stacks.pop(), None);

        set_of_stacks.push(1);
        set_of_stacks.push(1);
        set_of_stacks.push(1);
        set_of_stacks.push(1);

        assert_eq!(set_of_stacks.pop_at(0), Some(1));
        assert_eq!(set_of_stacks.pop_at(0), Some(1));
        assert_eq!(set_of_stacks.pop_at(1), Some(1));
        assert_eq!(set_of_stacks.pop_at(0), Some(1));

        assert_eq!(set_of_stacks.pop(), None);
    }
}
