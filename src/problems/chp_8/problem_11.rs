pub fn count_ways_change(n: usize, coins: &[u64]) -> u64 {
    if coins.len() == 0 || n == 0 {
        return 0;
    }
    fn helper(rem: usize, coin_idx: i64, memo: &mut [Vec<u64>], coins: &[u64]) -> u64 {
        if rem == 0 {
            return 1;
        }

        if coin_idx < 0 {
            return 0;
        }

        let index = coin_idx as usize;
        if memo[rem][index] != 0 {
            return memo[rem][index];
        }

        let coin_value = coins[index] as i64;
        let mut ways = 0u64;
        for coin_count in 0.. {
            if coin_value * coin_count > rem as i64 {
                break;
            }
            let rem = rem - (coin_value * coin_count) as usize;
            ways += helper(rem, coin_idx - 1, memo, coins);
        }

        memo[rem][index] = ways;
        ways
    }

    let mut memo = vec![vec![0u64; coins.len()]; n + 1];
    helper(n, coins.len() as i64 - 1, &mut memo, coins)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_ways_change_1() {
        let coins = [1, 5, 10, 25];
        assert_eq!(count_ways_change(0, &coins), 0);

        assert_eq!(count_ways_change(1, &coins), 1);
        assert_eq!(count_ways_change(2, &coins), 1);
        assert_eq!(count_ways_change(3, &coins), 1);
        assert_eq!(count_ways_change(4, &coins), 1);
        assert_eq!(count_ways_change(5, &coins), 2);
        assert_eq!(count_ways_change(6, &coins), 2);
        assert_eq!(count_ways_change(10, &coins), 4);
        assert_eq!(count_ways_change(25, &coins), 13);
    }
}
