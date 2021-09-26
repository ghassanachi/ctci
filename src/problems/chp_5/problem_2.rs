#[derive(Debug, PartialEq, Eq)]
pub enum BinStrErr {
    OutOfBounds,
    CannotRepresent,
}

pub fn binary_to_string(mut num: f64) -> Result<String, BinStrErr> {
    if num < 0.0 || num >= 1.0 {
        return Err(BinStrErr::OutOfBounds);
    }

    let mut divisor = 0.50;
    let mut output = String::from("0.");

    while num > 0.0 {
        println!("{}", num);
        if output.len() > 34 {
            return Err(BinStrErr::CannotRepresent);
        }
        if num >= divisor {
            output.push('1');
            num -= divisor;
        } else {
            output.push('0');
        }
        divisor /= 2.0;
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_to_string_1() {
        assert_eq!(binary_to_string(0.625), Ok("0.101".to_string()));
        assert_eq!(binary_to_string(0.25), Ok("0.01".to_string()));
        assert_eq!(binary_to_string(0.3125), Ok("0.0101".to_string()));
        assert_eq!(binary_to_string(0.0625), Ok("0.0001".to_string()));
    }

    #[test]
    fn binary_to_string_2() {
        assert_eq!(binary_to_string(-0.5), Err(BinStrErr::OutOfBounds));
        assert_eq!(binary_to_string(0.72), Err(BinStrErr::CannotRepresent));
    }
}
