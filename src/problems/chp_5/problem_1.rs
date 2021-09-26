/// Slow but easy
fn count_bits(num: u32) -> u8 {
    let mut len = 0u8;
    for position in 0..31 {
        if get_bit(num, position) {
            len = position
        }
    }
    len
}

pub fn get_bit(num: u32, position: u8) -> bool {
    return (num & (1 << position)) != 0;
}

pub fn bit_num_insertion(n: &mut u32, m: u32, i: u8, j: u8) {
    assert!(i < j, "i should be smaller than j");
    assert!(count_bits(m) <= j - i, "j - i != bit_len(m)");

    // create mask of all 0's up to j (ie: 11110000)
    let top_mask = !0 << j - 1;

    // create mask of all 1's up to i (ie: 00001111)
    let bottom_mask = (1 << i) - 1;

    let mask = top_mask | bottom_mask;

    // Clear bits from i to j;
    *n = *n & mask;

    // Shift m to i;
    let m = m << i;

    *n = *n | m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bit_helpers_test() {
        let num = 6u32;
        assert!(!get_bit(num, 0));
        assert!(get_bit(num, 1));
        assert!(get_bit(num, 2));
        assert!(!get_bit(num, 3));
    }

    /// Example from the ctci book
    #[test]
    fn bit_num_insertion_1() {
        let mut n = 0b10000000000u32;
        let m = 0b10011u32;
        let i = 2;
        let j = 6;
        bit_num_insertion(&mut n, m, i, j);
        assert_eq!(n, 0b10001001100);
    }

    /// test should fail since there is not enough room to fit m
    #[test]
    #[should_panic]
    fn bit_num_insertion_2() {
        let mut n = 1;
        let m = 0b10011u32;
        let i = 2;
        let j = 5;
        bit_num_insertion(&mut n, m, i, j);
        assert_eq!(n, 0b10001001100);
    }

    /// Example from the ctci book
    #[test]
    fn bit_num_insertion_3() {
        let mut n = 0u32;
        let m = 0b111u32;
        let i = 15;
        let j = 17;
        bit_num_insertion(&mut n, m, i, j);
        assert_eq!(n, m << 15);
    }
}
