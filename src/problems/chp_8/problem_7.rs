
pub fn hanoi_tower(num_disk: usize) -> (Vec<usize>, Vec<usize>, Vec<usize>) {
    let mut source: Vec<usize> = (0..num_disk).rev().collect();
    let mut buffer: Vec<usize> = Vec::with_capacity(num_disk);
    let mut target: Vec<usize> = Vec::with_capacity(num_disk);
    hanoi_tower_helper(source.len(), &mut source, &mut target, &mut buffer);
    (source, buffer, target)
}

fn hanoi_tower_helper(
    n: usize,
    source: &mut Vec<usize>,
    target: &mut Vec<usize>,
    buffer: &mut Vec<usize>,
) {
    if n > 0 {
        hanoi_tower_helper(n - 1, source, buffer, target);
        target.push(source.pop().unwrap());
        hanoi_tower_helper(n - 1, buffer, target, source);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_sorted<T>(data: &[T]) -> bool
    where
        T: Ord,
    {
        data.windows(2).all(|w| w[0] <= w[1])
    }

    #[test]
    fn tower_of_hanoi_1() {
        let (source, buffer, mut target) = hanoi_tower(8);
        assert!(source.is_empty());
        assert!(buffer.is_empty());
        assert_eq!(target.len(), 8);
        target.reverse();
        assert!(is_sorted(&target));
    }
}
