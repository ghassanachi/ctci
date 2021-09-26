#[allow(dead_code)]
fn get_bit(num: u32, position: u8) -> bool {
    return (num & (1 << position)) != 0;
}

pub fn flip_to_win(num: u32) -> u8 {
    let mut can_fill = true;
    let (mut i, mut j, mut current, mut max) = (0u8, 0u8, 0u8, 0u8);
    while j < 31 && i < 31 {
        match (get_bit(num, j), get_bit(num, i), can_fill) {
            (true, _, _) => {
                current += 1;
                max = max.max(current);
                j += 1;
            }
            (false, _, true) => {
                can_fill = false;
                current += 1;
                max = max.max(current);
                j += 1;
            }
            (false, true, false) => i += 1,
            (false, false, false) => {
                current = 0;
                can_fill = true;
                i += 1;
                j += 1;
            }
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flip_to_win_1() {
        assert_eq!(flip_to_win(0b1110111), 7);
        assert_eq!(flip_to_win(0b111011101111), 8);
        assert_eq!(flip_to_win(0b1110111001111), 7);
        assert_eq!(flip_to_win(0), 1);
    }
}
