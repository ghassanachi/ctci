pub fn power_set<T: Copy>(arr: &[T]) -> Vec<Vec<T>> {
    let mut result: Vec<Vec<T>> = vec![vec![]];
    for &num in arr {
        let mut with_me = result.clone();
        with_me.iter_mut().for_each(|copy| copy.push(num));
        result.append(&mut with_me);
    }
    result
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn power_set_1() {
        assert_eq!(power_set::<i32>(&[]), vec![vec![]]);
        assert_eq!(power_set::<i32>(&[1]), vec![vec![], vec![1]]);
        assert_eq!(
            power_set::<i32>(&[1, 2]),
            vec![vec![], vec![1], vec![2], vec![1, 2]]
        );
        assert_eq!(
            power_set::<i32>(&[1, 2, 3]),
            vec![
                vec![],
                vec![1],
                vec![2],
                vec![1, 2],
                vec![3],
                vec![1, 3],
                vec![2, 3],
                vec![1, 2, 3]
            ]
        )
    }
}
