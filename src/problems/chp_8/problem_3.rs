pub fn magic_index_unique(arr: &[i64]) -> Option<usize> {
    match arr.len() {
        0 => None,
        _ => magic_index_unique_helper(arr, 0, arr.len() - 1),
    }
}

fn magic_index_unique_helper(arr: &[i64], low: usize, high: usize) -> Option<usize> {
    if low == high && arr[low] != low as i64 {
        return None;
    }
    let mid = (high + low) / 2;
    if arr[mid] == mid as i64 {
        return Some(mid);
    }

    if arr[mid] > mid as i64 {
        return magic_index_unique_helper(arr, low, mid);
    }
    return magic_index_unique_helper(arr, mid + 1, high);
}

pub fn magic_index(arr: &[i64]) -> Option<usize> {
    match arr.len() {
        0 => None,
        _ => magic_index_helper(arr, 0, arr.len() - 1),
    }
}

fn magic_index_helper(arr: &[i64], low: usize, high: usize) -> Option<usize> {
    println!("low: {} high: {}", low, high);
    if low == high && arr[low] != low as i64 {
        return None;
    }

    let mid = (high + low) / 2;
    if arr[mid] == mid as i64 {
        return Some(mid);
    }

    if arr[mid] > mid as i64 {
        if let Some(val) = magic_index_helper(arr, low, mid) {
            return Some(val);
        }
        if let Some(val) = magic_index_helper(arr, mid + 1, arr[mid] as usize) {
            return Some(val);
        }
        return None;
    }

    if let Some(val) = magic_index_helper(arr, mid + 1, high) {
        return Some(val);
    }

    // If the number is negative then there is no need to check if it is magic since that would be
    // impossible
    if arr[mid].is_positive() {
        if let Some(val) = magic_index_helper(arr, arr[mid] as usize, mid) {
            return Some(val);
        }
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
