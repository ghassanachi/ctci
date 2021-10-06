#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Color {
    Red,
    Yellow,
    Blue,
    Green,
    Orange,
}

pub fn paint_fill<const N: usize, const M: usize>(
    screen: &mut [[Color; M]; N],
    position: (usize, usize),
    to: Color,
) {
    if N == 0 || M == 0 {
        return;
    }
    let (row, col) = position;
    let from = screen[row][col];
    if from == to {
        return;
    }
    paint_fill_helper(screen, position, from, to)
}

fn paint_fill_helper<const N: usize, const M: usize>(
    screen: &mut [[Color; M]; N],
    position: (usize, usize),
    from: Color,
    to: Color,
) {
    assert!(N != 0 && M != 0, "empty screen");

    let (row, col) = position;
    println!("row: {}, col: {}", row, col);
    screen[row][col] = to;

    if row + 1 < N && screen[row + 1][col] == from {
        paint_fill_helper(screen, (row + 1, col), from, to);
    }

    if row > 0 && screen[row - 1][col] == from {
        paint_fill_helper(screen, (row - 1, col), from, to);
    }

    if col + 1 < M && screen[row][col + 1] == from {
        paint_fill_helper(screen, (row, col + 1), from, to);
    }

    if col > 0 && screen[row][col - 1] == from {
        paint_fill_helper(screen, (row, col - 1), from, to);
    }
}

#[cfg(test)]
mod tests {
    use super::Color::*;
    use super::*;

    #[test]
    fn paint_fill_1() {
        let mut screen = [
            [Blue, Blue, Blue, Yellow],
            [Red, Blue, Blue, Blue],
            [Yellow, Blue, Red, Yellow],
        ];
        let expected = screen.clone();
        paint_fill(&mut screen, (0, 0), Blue);
        assert_eq!(screen, expected);

        paint_fill(&mut screen, (0, 0), Orange);
        assert_eq!(
            screen,
            [
                [Orange, Orange, Orange, Yellow],
                [Red, Orange, Orange, Orange],
                [Yellow, Orange, Red, Yellow],
            ]
        )
    }

    #[test]
    fn paint_fill_2() {
        let mut screen = [[]];
        paint_fill(&mut screen, (0, 0), Orange);
        assert_eq!(screen, [[]]);

        let mut screen = [[Red; 5]; 4];
        let expected = screen.clone();
        paint_fill(&mut screen, (1, 1), Red);
        assert_eq!(screen, expected);

        paint_fill(&mut screen, (1, 1), Orange);
        assert_eq!(screen, [[Orange; 5]; 4]);
    }

    #[test]
    fn paint_fill_3() {
        let mut screen = [
            [Red, Red, Blue, Red, Red],
            [Red, Red, Blue, Red, Red],
            [Blue, Blue, Blue, Red, Red],
            [Red, Red, Red, Red, Red],
            [Red, Red, Red, Red, Red],
        ];
        paint_fill(&mut screen, (0, 0), Blue);
        assert_eq!(
            screen,
            [
                [Blue, Blue, Blue, Red, Red],
                [Blue, Blue, Blue, Red, Red],
                [Blue, Blue, Blue, Red, Red],
                [Red, Red, Red, Red, Red],
                [Red, Red, Red, Red, Red],
            ]
        );

        let mut screen = [[Red; 5]; 4];
        let expected = screen.clone();
        paint_fill(&mut screen, (1, 1), Red);
        assert_eq!(screen, expected);

        paint_fill(&mut screen, (1, 1), Orange);
        assert_eq!(screen, [[Orange; 5]; 4]);
    }
}
