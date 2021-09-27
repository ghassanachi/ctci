/// Xor'ing the two numbers will only leave the bits that are different between n and m, and the
/// number of "high" bits is equivalent to the number of swaps needed to convert the numbers from
/// one to the other.
///
/// I'm using the build in count_ones() function since it is optmized, but getting the number of
/// high bits is constant over the length of the int or uint with a simple for loop if I had to
/// implement it.
pub fn count_bit_to_swap(n: u32, m: u32) -> u32 {
    (n ^ m).count_ones()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_bit_to_swap_1() {
        assert_eq!(count_bit_to_swap(0b1001, 0b1010), 2);
        assert_eq!(count_bit_to_swap(0b1111, 0b0000), 4);
        assert_eq!(count_bit_to_swap(1, 1), 0);
        assert_eq!(count_bit_to_swap(0b0100_1001, 0b1001_0010), 6);
    }
}
