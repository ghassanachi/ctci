pub fn recurse_mutiply(a: i64, mut b: i64) -> i64 {
    if a == 0 || b == 0 {
        return 0;
    }

    // Reverse order to have b as small as possible
    if b > a {
        return recurse_mutiply(b, a);
    }

    let mut out = a;
    let mut remainder = 0;

    while b > 1 {
        if b % 2 == 1 {
            remainder += out;
        }
        out += out;
        b /= 2;
    }

    out + remainder
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recurse_mutiply_1() {
        assert_eq!(recurse_mutiply(8, 9), 72);
        assert_eq!(recurse_mutiply(4, 4), 16);
        assert_eq!(recurse_mutiply(10, 0), 0);
        assert_eq!(recurse_mutiply(739, 129), 95_331);
        assert_eq!(recurse_mutiply(129, 739), 95_331);
        assert_eq!(recurse_mutiply(7, 8), 56);

        assert_eq!(recurse_mutiply(561_038_692_387, 7), 3_927_270_846_709);
        assert_eq!(recurse_mutiply(7, 561_038_692_387), 3_927_270_846_709);
    }
}
