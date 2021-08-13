use crate::structures::Stack;
use std::cmp::Ordering;
use std::fmt::Debug;

pub fn sort_stack<T: Debug + Copy + Eq + Ord + PartialEq + PartialOrd>(stack: &mut Stack<T>) {
    let mut rev_stack: Stack<T> = Stack::new();
    while let Some(stack_val) = stack.pop() {
        if let Some(rev_val) = rev_stack.peek() {
            match stack_val.cmp(rev_val) {
                Ordering::Less => {
                    let mut pop_count = 0;
                    while let Some(smaller) = rev_stack.peek() {
                        if stack_val < *smaller {
                            let smaller = rev_stack.pop().expect("we just peeked and got Some");
                            pop_count += 1;
                            stack.push(smaller);
                        } else {
                            break;
                        }
                    }
                    rev_stack.push(stack_val);
                    for _ in 0..pop_count {
                        let val = stack.pop().expect("stack was just filled");
                        rev_stack.push(val);
                    }
                }
                _ => {
                    rev_stack.push(stack_val);
                }
            }
        } else {
            rev_stack.push(stack_val)
        }
    }

    // stack is empty, rev_stack is  in reverse order
    while let Some(larger) = rev_stack.pop() {
        stack.push(larger)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::{Rng, SeedableRng};

    #[test]
    fn sort_stack_1() {
        let iter = vec![5, 4, 3, 2, 1];

        let mut expected = iter.clone();
        expected.sort();

        let mut stack = Stack::from_iter(iter);
        sort_stack(&mut stack);

        let result: Vec<_> = stack.into_iter().collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn sort_stack_2() {
        let iter = vec![2, 4, 1, 1, 2, 3, 5, 2, 5];

        let mut expected = iter.clone();
        expected.sort();

        let mut stack = Stack::from_iter(iter);
        sort_stack(&mut stack);

        let result: Vec<_> = stack.into_iter().collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn sort_stack_3() {
        let mut rng = StdRng::seed_from_u64(42);
        let iter: Vec<u64> = (0..10).map(|_| rng.gen_range(0..20)).collect();

        let mut expected = iter.clone();
        expected.sort();

        let mut stack = Stack::from_iter(iter);
        sort_stack(&mut stack);

        let result: Vec<_> = stack.into_iter().collect();
        assert_eq!(result, expected);
    }
}
