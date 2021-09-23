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

fn update_bit(num: &mut u32, position: u8, val: bool) {
    let value = if val { 1 } else { 0 };
    let mask = !(1 << position);
    *num = (*num & mask) | (value << position)
}

pub fn get_bit(num: u32, position: u8) -> bool {
    return (num & (1 << position)) != 0;
}

pub fn bit_num_insertion(n: &mut u32, m: u32, i: u8, j: u8) {
    assert!(i < j, "i should be smaller than j");
    assert!(count_bits(m) <= j - i, "j - i != bit_len(m)");
    for position in i..=j {
        let bit = get_bit(m, position - i);
        update_bit(n, position, bit);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bit_helpers_test() {
        let mut num = 6u32;
        assert!(!get_bit(num, 0));
        assert!(get_bit(num, 1));
        assert!(get_bit(num, 2));
        assert!(!get_bit(num, 3));
        update_bit(&mut num, 0, true);
        assert!(get_bit(num, 0));
        assert_eq!(num, 7);
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
        assert!(n != m << 18);
    }
}
