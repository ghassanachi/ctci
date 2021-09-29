use std::fmt::Display;
use std::ops::{Deref, DerefMut};

pub struct Screen<const N: usize> {
    array: [u8; N],
    width: usize,
}

impl<const N: usize> Screen<N> {
    pub fn new(width: usize) -> Self {
        assert!(N % width == 0);
        Screen {
            array: [0; N],
            width,
        }
    }
}

impl<const N: usize> Deref for Screen<N> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.array
    }
}

impl<const N: usize> DerefMut for Screen<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.array
    }
}

impl<const N: usize> Display for Screen<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.iter();
        let mut dividor = None;

        while iter.len() != 0 {
            let mut row = Vec::new();
            for _ in 0..self.width {
                row.push(format!("{:08b}", iter.next().unwrap()));
            }
            let row = "| ".to_owned() + &row.join(" | ") + " |";

            dividor = dividor.or_else(|| Some(String::from("-").repeat(row.len())));

            writeln!(f, "{}", dividor.as_ref().unwrap()).ok();
            writeln!(f, "{}", row).ok();
        }

        if let Some(dividor) = dividor {
            return writeln!(f, "{}", dividor);
        }
        writeln!(f, "Empty screen")
    }
}

pub fn draw_line<const N: usize>(
    screen: &mut Screen<N>,
    width: usize,
    x1: usize,
    x2: usize,
    y: usize,
) {
    let row_start = width * y;
    let row = &mut screen[row_start + (x1 / 8)..=row_start + (x2 / 8)];
    let first_mask = !0 >> (x1 % 8) as u8;
    let last_mask = !(!0 >> ((x2 + 1) % 8));

    if let Some(first) = row.first_mut() {
        // Early return since x1 and xy are in the same byte
        if x2 / 8 == x1 / 8 {
            *first = last_mask & first_mask;
            return;
        }
        *first = first_mask;
    }

    if let Some(last) = row.last_mut() {
        *last = last_mask;
    }

    if row.len() > 2 {
        let middle_end = row.len() - 1;
        for middle in &mut row[1..middle_end] {
            *middle = !0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn draw_line_1() {
        let width = 3;
        let mut screen = Screen::<6>::new(width);
        draw_line(&mut screen, width, 8, 16, 0);
        assert_eq!(screen.array, [0, 0b1111_1111, 0b1000_0000, 0, 0, 0]);
    }

    #[test]
    fn draw_line_2() {
        let width = 3;
        let mut screen = Screen::<6>::new(width);
        draw_line(&mut screen, width, 8, 8, 0);
        assert_eq!(screen.array, [0, 0b1000_0000, 0, 0, 0, 0]);
        let mut screen = Screen::<6>::new(width);
        draw_line(&mut screen, width, 10, 12, 0);
        assert_eq!(screen.array, [0, 0b0011_1000, 0, 0, 0, 0]);
    }

    #[test]
    fn draw_line_3() {
        let width = 7;
        let mut screen = Screen::<49>::new(width);
        draw_line(&mut screen, width, 0, 56, 3);
        assert_eq!(screen.array[0..7], [0; 7]);
        assert_eq!(screen.array[14..21], [0; 7]);
        assert_eq!(screen.array[21..28], [!0; 7]);
        assert_eq!(screen.array[35..42], [0; 7]);
    }

    #[test]
    fn draw_line_4() {
        let width = 16;
        let mut screen = Screen::<256>::new(width);
        draw_line(&mut screen, width, 26, 110, 8);
        assert_eq!(screen.array[0..16], [0; 16]);
        assert_eq!(
            screen.array[128..144],
            [
                0,
                0,
                0,
                0b0011_1111,
                !0,
                !0,
                !0,
                !0,
                !0,
                !0,
                !0,
                !0,
                !0,
                0b1111_1110,
                0,
                0,
            ]
        );
        assert_eq!(screen.array[48..64], [0; 16]);
    }
}
