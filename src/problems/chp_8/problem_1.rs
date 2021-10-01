pub fn count_num_way_up(num_steps: usize) -> usize {
    let mut dp = vec![0; num_steps + 1];
    dp[0] = 1;
    for i in 1..=num_steps {
        dp[i] += dp[i - 1];
        if i as i32 - 2 >= 0 {
            dp[i] += dp[i - 2]
        }
        if i as i32 - 3 >= 0 {
            dp[i] += dp[i - 3]
        }
    }
    return dp[num_steps];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_num_way_up_1() {
        assert_eq!(count_num_way_up(1), 1);
        assert_eq!(count_num_way_up(2), 2);
        assert_eq!(count_num_way_up(3), 4);
        assert_eq!(count_num_way_up(4), 7);
        assert_eq!(count_num_way_up(5), 13);
        assert_eq!(count_num_way_up(6), 24);
    }
}
