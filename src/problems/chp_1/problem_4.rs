pub fn palindrome_permutation(input: &str) -> bool {
    let mut bit_map: u64 = 0;
    for c in input.chars() {
        bit_map ^= 1 << (c as u32 - 'a' as u32)
    }

    let mut has_single = false;
    for i in 0..64 {
        if bit_map & (1 << i as u32) != 0 {
            // Even lengths makes odd occurrences impossible
            if input.len() % 2 == 0 {
                return false;
            }
            // Odd lengths can't have more than one single
            if has_single {
                return false;
            }
            has_single = true;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palindrome_permutation_1() {
        assert!(palindrome_permutation("ollo"));
    }

    #[test]
    fn test_palindrome_permutation_2() {
        assert!(!palindrome_permutation("ooolll"));
    }

    #[test]
    fn test_palindrome_permutation_3() {
        assert!(palindrome_permutation("racecar"));
    }
}
