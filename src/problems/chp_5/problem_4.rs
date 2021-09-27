pub fn prev_and_next(num: u32) -> (Option<u32>, Option<u32>) {
    return (get_next(!num).map(|res| !res), get_next(num));
}
fn get_next(mut num: u32) -> Option<u32> {
    if num == 0 {
        return None;
    }

    let mut zero_count = 0;
    let mut switch_position: Option<u8> = None;
    for position in 0..31 {
        if (num & (1 << position)) != 0 {
            continue;
        }
        if zero_count != position {
            switch_position = Some(position);
            break;
        }
        zero_count += 1;
    }

    let switch_position = switch_position?;

    // subtract one since we are about to perform the switch;
    let one_count = switch_position - zero_count - 1;
    // Set the 1
    num = num | (1 << switch_position);

    // Clear all bottom bits from shift position
    num = num & !((1 << switch_position) - 1);

    Some(num | (1 << one_count) - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prev_and_next_1() {
        assert_eq!(prev_and_next(0b10), (Some(0b1), Some(0b100)));
        assert_eq!(prev_and_next(0b1010), (Some(0b1001), Some(0b1100)));
        // Rust ctci example
        assert_eq!(
            prev_and_next(0b11011001111100),
            (Some(0b11011001111010), Some(0b11011010001111))
        )
    }

    #[test]
    fn prev_and_next_2() {
        // All 0's cannot have a next or a prev
        assert_eq!(prev_and_next(0), (None, None));
        // All 1's cannot have a next or a prev
        assert_eq!(prev_and_next(!0), (None, None));
        // Binary 0b000..001 cannot have a prev
        assert_eq!(prev_and_next(1), (None, Some(0b10)));
        // Binary 0b111..110 cannot have a next
        assert_eq!(prev_and_next(!0 - 1), (Some(!0 - 2), None));
    }
}
