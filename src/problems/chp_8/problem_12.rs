use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct QueenBoard<const N: usize> {
    board: [[Piece; N]; N],
}

impl<const N: usize> QueenBoard<N> {
    pub fn new() -> Self {
        Self {
            board: [[Piece::Empty; N]; N],
        }
    }

    fn diagonals(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        assert!(row < N, "row {} is greater than size of board {}", row, N);
        assert!(col < N, "col {} is greater than size of board {}", col, N);

        let row = row as i64;
        let col = col as i64;
        let size = N as i64;
        let diagonals = (1i64..size)
            .map(|d| {
                [
                    (row - d, col - d),
                    (row + d, col + d),
                    (row - d, col + d),
                    (row + d, col - d),
                ]
            })
            .flatten()
            .filter(|cell| cell.0 >= 0 && cell.0 < size && cell.1 >= 0 && cell.1 < size)
            .map(|cell| (cell.0 as usize, cell.1 as usize));
        diagonals.collect()
    }

    fn check_valid(&mut self, row: usize, col: usize) -> bool {
        if row >= N || col >= N {
            return false;
        }

        for r in 0..N {
            if self.board[r][col] == Piece::Queen {
                return false;
            }
        }

        for (r, c) in self.diagonals(row, col) {
            if self.board[r][c] == Piece::Queen {
                return false;
            }
        }
        return true;
    }

    pub fn valid_boards(&mut self) -> Vec<Self> {
        let mut results: Vec<Self> = Vec::new();
        self.valid_boards_helper(0, &mut results);
        results
    }

    fn valid_boards_helper(&mut self, row: usize, results: &mut Vec<Self>) {
        if row == N {
            let board = self.clone();
            results.push(board);
            return;
        }

        for col in 0..N {
            if self.check_valid(row, col) {
                self.board[row][col] = Piece::Queen;
                self.valid_boards_helper(row + 1, results);
                self.board[row][col] = Piece::Empty;
            }
        }
    }
}

impl<const N: usize> Display for QueenBoard<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dividor = "-".repeat((self.board.len() - 1) * 4 + 5);
        for row in &self.board {
            let row: Vec<String> = row.iter().map(|piece| format!("{}", piece)).collect();
            let row = "| ".to_owned() + &row.join(" | ") + " |";
            writeln!(f, "{}", dividor)?;
            writeln!(f, "{}", row)?;
        }
        writeln!(f, "{}", dividor)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Piece {
    Queen,
    Empty,
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Piece::Empty => write!(f, " "),
            Piece::Queen => write!(f, "♕"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Instead of validating that all boards are the same, I am checking for expected length of
    /// results based on the following table:
    /// http://www.durangobill.com/N_Queens.html
    fn valid_queens_test() {
        let mut board: QueenBoard<1> = QueenBoard::new();
        let valid_boards = board.valid_boards();
        assert_eq!(valid_boards.len(), 1);

        let mut board: QueenBoard<2> = QueenBoard::new();
        let valid_boards = board.valid_boards();
        assert_eq!(valid_boards.len(), 0);

        let mut board: QueenBoard<4> = QueenBoard::new();
        let valid_boards = board.valid_boards();
        assert_eq!(valid_boards.len(), 2);

        let mut board: QueenBoard<5> = QueenBoard::new();
        let valid_boards = board.valid_boards();
        assert_eq!(valid_boards.len(), 10);

        let mut board: QueenBoard<6> = QueenBoard::new();
        let valid_boards = board.valid_boards();
        assert_eq!(valid_boards.len(), 4);

        let mut board: QueenBoard<7> = QueenBoard::new();
        let valid_boards = board.valid_boards();
        assert_eq!(valid_boards.len(), 40);

        let mut board: QueenBoard<8> = QueenBoard::new();
        let valid_boards = board.valid_boards();
        assert_eq!(valid_boards.len(), 92);

        let mut board: QueenBoard<9> = QueenBoard::new();
        let valid_boards = board.valid_boards();
        assert_eq!(valid_boards.len(), 352);
    }
}
