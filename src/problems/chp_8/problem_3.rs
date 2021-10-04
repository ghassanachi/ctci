pub fn magic_index_unique(arr: &[i64]) -> Option<usize> {
    magic_index_unique_helper(arr, 0, (arr.len() as i64) - 1)
}

fn magic_index_unique_helper(arr: &[i64], low: i64, high: i64) -> Option<usize> {
    if low > high {
        return None;
    }
    let mid = (high + low) / 2;
    match arr[mid as usize].cmp(&mid) {
        std::cmp::Ordering::Less => magic_index_unique_helper(arr, mid + 1, high),
        std::cmp::Ordering::Equal => Some(mid as usize),
        std::cmp::Ordering::Greater => magic_index_unique_helper(arr, low, mid - 1),
    }
}

pub fn magic_index(arr: &[i64]) -> Option<usize> {
    magic_index_helper(arr, 0, (arr.len() as i64) - 1)
}

fn magic_index_helper(arr: &[i64], low: i64, high: i64) -> Option<usize> {
    if low > high {
        return None;
    }

    let mid_index = (high + low) / 2;
    let mid_val = arr[mid_index as usize];
    if mid_val == mid_index {
        return Some(mid_index as usize);
    }

    // Search left side from up to the min of the previous index or the mid value index if smaller.
    let left_end = mid_val.min(mid_index - 1);
    if let Some(val) = magic_index_helper(arr, low, left_end) {
        return Some(val);
    }

    // Search right side from next index or the mid value index if it is bigger.
    let right_start = mid_val.max(mid_index + 1);
    if let Some(val) = magic_index_helper(arr, right_start, high) {
        return Some(val);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn magic_index_unique_1() {
        assert_eq!(magic_index_unique(&[-1, 0, 2, 4, 5]), Some(2));
        assert_eq!(magic_index_unique(&[-1, 0, 1, 2, 4]), Some(4));
        assert_eq!(magic_index_unique(&[0, 2, 3, 4, 5]), Some(0));
        assert_eq!(
            magic_index_unique(&[-2, -1, 2, 4, 5, 6, 7, 8, 9, 10, 11]),
            Some(2)
        );
        assert_eq!(magic_index_unique(&[-1, 0, 1, 2, 3]), None);
        assert_eq!(magic_index_unique(&[]), None);
    }

    #[test]
    fn magic_index_1() {
        assert_eq!(magic_index(&[-1, 0, 2, 4, 5]), Some(2));
        assert_eq!(magic_index(&[-1, 0, 1, 2, 4]), Some(4));
        assert_eq!(magic_index(&[0, 2, 3, 4, 5]), Some(0));
        assert_eq!(magic_index(&[-2, -1, 2, 4, 5, 6, 7, 8, 9, 10, 11]), Some(2));
        assert_eq!(magic_index(&[-1, 0, 1, 2, 3]), None);
        assert_eq!(magic_index(&[]), None);

        assert_eq!(magic_index(&[3, 3, 3, 3, 3]), Some(3));
        assert_eq!(magic_index(&[-1, -1, -1, -1, -1, -1]), None);
        assert_eq!(magic_index(&[-1, -1, -1, -1, 4, -1]), Some(4));
    }
}
