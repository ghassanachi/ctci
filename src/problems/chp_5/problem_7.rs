const LEFT_MASK: u32 = 0b1010_1010_1010_1010_1010_1010_1010_1010;
const RIGHT_MASK: u32 = 0b0101_0101_0101_0101_0101_0101_0101_0101;

pub fn pairwise_swap(num: u32) -> u32 {
    let right_shift = (num >> 1) & RIGHT_MASK;
    let left_shift = (num << 1) & LEFT_MASK;

    return right_shift | left_shift;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pairwise_swap_1() {
        assert_eq!(pairwise_swap(0b1010), 0b0101);
        assert_eq!(pairwise_swap(0b110), 0b1001);
        assert_eq!(pairwise_swap(0), 0);
        assert_eq!(pairwise_swap(!0), !0);
    }
}
