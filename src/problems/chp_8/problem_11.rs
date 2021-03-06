/// This is a similar approach to a sieve.
/// For each coin:
/// 1. Loop from coin_value to amount
///     a. Increment the dp array at index i by dp[i - coin], this is equavalent to taking
///     0..=(amount/coin_value) coins given how it updates the array.
/// 2. Return dp[amount]
pub fn count_ways_change(amount: usize, coins: &[usize]) -> usize {
    let mut dp = vec![0; amount + 1];
    dp[0] = 1;

    for &coin in coins {
        for i in coin..=amount {
            dp[i] += dp[i - coin];
        }
    }

    dp[amount]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_ways_change_1() {
        let coins = [1, 5, 10, 25];
        assert_eq!(count_ways_change(0, &coins), 1);

        assert_eq!(count_ways_change(1, &coins), 1);
        assert_eq!(count_ways_change(2, &coins), 1);
        assert_eq!(count_ways_change(3, &coins), 1);
        assert_eq!(count_ways_change(4, &coins), 1);
        assert_eq!(count_ways_change(5, &coins), 2);
        assert_eq!(count_ways_change(6, &coins), 2);
        assert_eq!(count_ways_change(7, &coins), 2);
        assert_eq!(count_ways_change(10, &coins), 4);
        assert_eq!(count_ways_change(25, &coins), 13);
    }
}
