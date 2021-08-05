use std::fmt::Debug;

#[derive(Debug, Clone, Copy)]
struct StackInfo {
    start: usize,
    size: usize,
}

pub struct MultiStack<T> {
    array: Vec<Option<T>>,
    stack_info: Vec<StackInfo>,
}

impl<T> MultiStack<T>
where
    T: Clone + Debug,
{
    pub fn new(num_stack: usize, stack_capacity: usize) -> Self {
        let array: Vec<Option<T>> = vec![None; num_stack * stack_capacity];
        let mut stack_info: Vec<StackInfo> = Vec::with_capacity(num_stack);

        for n in 0..num_stack {
            stack_info.push(StackInfo {
                start: n * stack_capacity,
                size: 0,
            });
        }

        return Self { array, stack_info };
    }

    pub fn push(&mut self, stack: usize, value: T) -> Result<(), &'static str> {
        if self.stack_at_capacity(stack) {
            self.grow(stack)?;
        }
        let cur_info = &self.stack_info[stack];
        let index = self.rebase_index(cur_info.start + cur_info.size);
        self.array[index] = Some(value);
        self.stack_info[stack].size += 1;
        Ok(())
    }

    pub fn pop(&mut self, stack: usize) -> Option<T> {
        let cur_info = &self.stack_info[stack];
        if !self.is_empty(stack) {
            let index = self.rebase_index(cur_info.start + cur_info.size - 1);
            let out = self.array[index].take();
            self.stack_info[stack].size -= 1;
            return out;
        }
        None
    }

    fn rebase_index(&self, index: usize) -> usize {
        index % self.array.len()
    }

    fn get_stack(&self, stack: usize) -> StackInfo {
        self.stack_info[stack % self.stack_info.len()]
    }

    fn stack_at_capacity(&self, stack: usize) -> bool {
        let cur_stack = self.get_stack(stack);
        let next_stack = self.get_stack(stack + 1);
        self.rebase_index(cur_stack.start + cur_stack.size) == next_stack.start
    }

    pub fn is_full(&self, stack: usize) -> bool {
        if !self.stack_at_capacity(stack) {
            return false;
        }

        for i in 0..self.stack_info.len() {
            let cur_stack = self.get_stack(i);
            let next_stack = self.get_stack(i + 1);
            if self.rebase_index(cur_stack.start + cur_stack.size) != next_stack.start {
                return false;
            }
        }
        true
    }

    pub fn is_empty(&self, stack: usize) -> bool {
        self.stack_info[stack].size == 0
    }

    fn increment_pointer(pointer: &mut usize, len: usize) {
        if *pointer == len - 1 {
            *pointer = 0;
        } else {
            *pointer += 1;
        }
    }

    fn grow(&mut self, stack: usize) -> Result<(), &'static str> {
        let (ahead, behind): (Vec<usize>, Vec<usize>) = self
            .stack_info
            .iter()
            .enumerate()
            .map(|(i, _)| i)
            .filter(|i| *i != stack)
            .partition(|i| *i > stack);

        let iter = ahead.iter().chain(behind.iter());

        let mut rev_idx: Option<usize> = None;

        for (i, &stack_index) in iter.enumerate() {
            let info = self.get_stack(stack_index);
            let next_stack = self.get_stack(stack_index + 1);
            if self.rebase_index(info.start + info.size) != next_stack.start {
                rev_idx = Some(i);
                break;
            }
        }

        if rev_idx.is_none() {
            return Err("Stacks are all full");
        }

        let rev_idx = rev_idx.unwrap();

        let resize_array: Vec<usize> = ahead
            .iter()
            .chain(behind.iter())
            .enumerate()
            .filter(|(i, _)| i <= &rev_idx)
            .map(|(_, s)| *s)
            .collect();

        let mut iter = resize_array.iter();

        while let Some(&index) = iter.next_back() {
            let info = self.get_stack(index);

            match (info.size, info.start + info.size < self.array.len()) {
                (0, _) => {}
                (_, true) => self.array[info.start..=(info.start + info.size)].rotate_right(1),
                (_, false) => {
                    let rebased = self.rebase_index(info.start + info.size);
                    if rebased > 0 {
                        self.array[..rebased].rotate_right(1)
                    }
                    if self.array.len() - info.start > 1 {
                        self.array[info.start..].rotate_right(1)
                    }
                    self.array.swap(0, info.start);
                }
            }
            let mut inner_start = self.stack_info[index].start;
            Self::increment_pointer(&mut inner_start, self.array.len());
            self.stack_info[index].start = inner_start;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_in_one_1() {
        let mut multi_stack = MultiStack::new(3, 3);

        assert!(multi_stack.is_empty(0));
        assert!(multi_stack.push(0, 1).is_ok());
        assert!(multi_stack.push(0, 2).is_ok());
        assert!(multi_stack.push(0, 3).is_ok());
        assert!(!multi_stack.is_full(0));

        assert!(multi_stack.is_empty(1));
        assert!(multi_stack.push(1, 4).is_ok());
        assert!(multi_stack.push(1, 5).is_ok());
        assert!(multi_stack.push(1, 6).is_ok());
        assert!(!multi_stack.is_full(1));

        assert!(multi_stack.is_empty(2));
        assert!(multi_stack.push(2, 7).is_ok());
        assert!(multi_stack.push(2, 8).is_ok());
        assert!(multi_stack.push(2, 9).is_ok());
        assert!(multi_stack.is_full(2));

        assert!(multi_stack.is_full(0));
        assert!(multi_stack.is_full(1));

        assert!(multi_stack.push(0, 10).is_err());
        assert!(multi_stack.push(1, 10).is_err());
        assert!(multi_stack.push(2, 10).is_err());

        assert_eq!(multi_stack.pop(0), Some(3));
        assert_eq!(multi_stack.pop(1), Some(6));
        assert_eq!(multi_stack.pop(2), Some(9));

        assert!(!multi_stack.is_full(0));
        assert!(!multi_stack.is_full(1));

        assert!(!multi_stack.is_empty(2));
        assert!(multi_stack.push(1, 10).is_ok());
        assert!(multi_stack.push(1, 11).is_ok());
        assert!(multi_stack.push(1, 12).is_ok());
        assert!(multi_stack.is_full(2));

        assert_eq!(multi_stack.pop(0), Some(2));
        assert_eq!(multi_stack.pop(1), Some(12));
        assert_eq!(multi_stack.pop(2), Some(8));

        assert_eq!(multi_stack.pop(0), Some(1));
        assert_eq!(multi_stack.pop(1), Some(11));
        assert_eq!(multi_stack.pop(2), Some(7));

        assert_eq!(multi_stack.pop(0), None);
        assert_eq!(multi_stack.pop(1), Some(10));
        assert_eq!(multi_stack.pop(2), None);
    }

    #[test]
    fn three_in_one_2() {
        let mut multi_stack = MultiStack::new(3, 3);

        assert!(multi_stack.is_empty(0));
        assert!(multi_stack.push(0, 1).is_ok());
        assert!(multi_stack.push(0, 2).is_ok());
        assert!(multi_stack.push(0, 3).is_ok());
        assert!(multi_stack.push(0, 4).is_ok());
        assert!(multi_stack.push(0, 5).is_ok());
        assert!(multi_stack.push(0, 6).is_ok());
        assert!(multi_stack.push(0, 7).is_ok());
        assert!(multi_stack.push(0, 8).is_ok());
        assert!(multi_stack.push(0, 9).is_ok());
        assert!(multi_stack.is_full(0));

        assert!(multi_stack.push(0, 10).is_err());
        assert_eq!(multi_stack.pop(0), Some(9));
        assert_eq!(multi_stack.pop(0), Some(8));

        assert!(multi_stack.push(1, 10).is_ok());
        assert!(multi_stack.push(2, 11).is_ok());

        assert_eq!(multi_stack.pop(0), Some(7));
        assert_eq!(multi_stack.pop(1), Some(10));
        assert_eq!(multi_stack.pop(2), Some(11));
    }
}
