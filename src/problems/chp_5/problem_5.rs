/// For (num & (num - 1)) to evaluate to false, all bits must flip. This is only true when the bit
/// pattern has 0 or 1 bits set to 1. Thus this expression checks if the number is a power of 2 or
/// 0.
///
/// ## Example: (0b00100 - 1) => (0b00011)
pub fn bit_wut(num: i32) -> bool {
    (num & (num - 1)) == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bit_wut_1() {
        assert!(bit_wut(0));
        assert!(bit_wut(1));
        assert!(bit_wut(2));
        assert!(bit_wut(4));
        assert!(bit_wut(16));
        assert!(bit_wut(256));
        assert!(bit_wut(1024));

        assert!(!bit_wut(3));
        assert!(!bit_wut(5));
        assert!(!bit_wut(9));
        assert!(!bit_wut(20));
        assert!(!bit_wut(134));
        assert!(!bit_wut(573));
        assert!(!bit_wut(1099));
    }
}
